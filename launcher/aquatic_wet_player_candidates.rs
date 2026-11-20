fn aquatic_wet_player_candidates(_result: (), game: &mut Game<'_>, snapshot: &[Seen]) {
    use horizon_client::mobs::{
        PlayerView, Species, Who, goals::naturalist_alligator_goals::{self as alligator, near_eggs},
        nav::{naturalist_catfish_state as hunt, naturalist_hippo_state as hippo,
            naturalist_piranha_state as piranha, naturalist_shark_state as shark},
    };
    use horizon_core::geometry::Vec3;

    const ALLIGATOR: &str = "naturalist:alligator";
    const PIRANHA: &str = "naturalist:piranha";
    let hunts_swimmers = |mob: &Mob| matches!(mob.kind.id.as_str(), shark::KIND | PIRANHA | ALLIGATOR | hippo::KIND) && mob.alive();
    if !game.mobs.iter().any(hunts_swimmers) {
        return;
    }
    let guests = game.lan_guest_views();
    let host = game.player_view();
    let (choices, nest_actors) = {
        let world = game.mob_world_base(snapshot);
        let players = || {
            host.iter().map(|player| (None, player))
                .chain(guests.iter().map(|(conn, player)| (Some(*conn), player)))
        };
        let nest_actors = if !guests.is_empty() && game.mobs.iter().any(alligator::pursuing_swimmer) {
            players().filter(|(_, player)| player.alive && !player.creative && !player.spectator
                && near_eggs(&world, player.position))
                .map(|(conn, player)| (conn, player.clone()))
                .collect::<std::sync::Arc<[_]>>()
        } else {
            std::sync::Arc::from([])
        };
        let live_actor = |mob| {
            let connection = game.lan.as_ref().and_then(|lan| lan.mob_players.get(&mob)).copied();
            players().any(|(actor, player)| actor == connection && player.alive && !player.spectator)
                .then_some(connection)
        };
        let school_leader = |mob: &Mob| mob.extra.leader.and_then(|id| world.seen(id)).filter(|seen| seen.alive);
        let is_piranha = |seen: &Seen| seen.species == Species::Cod && seen.variant == piranha::SNAPSHOT;
        // The follower's pending damage event carries the target kind. Preserve its actor too.
        let alerts: std::collections::HashMap<_, _> = game.mobs.iter().filter_map(|mob| {
            if mob.kind.id != PIRANHA || !piranha::has_player_retaliation(&mob.ai.nav) {
                return None;
            }
            let leader = school_leader(mob).filter(|seen| is_piranha(seen))?;
            Some((leader.id, live_actor(mob.id)?))
        }).collect();
        let choices = game.mobs.iter().enumerate()
            .filter(|(_, mob)| hunts_swimmers(mob))
            .map(|(index, mob)| {
                let retaliating = mob.ai.last_hurt_by == Some(Who::Player)
                    && mob.age as i32 - mob.ai.last_hurt_by_time <= 100;
                if guests.is_empty() || mob.no_ai || mob.extra.leash.is_some() {
                    return (index, None);
                }
                let kind = mob.kind.id.as_str();
                if kind == PIRANHA && !piranha::has_player_retaliation(&mob.ai.nav) {
                    if let Some(&connection) = alerts.get(&mob.id) {
                        return (index, Some(connection));
                    }
                    if let Some(leader) = school_leader(mob) {
                        // A follower copies the leader's player target and its actor together.
                        let choice = if is_piranha(leader) {
                            alerts.get(&leader.id).copied().or_else(|| {
                                (leader.target == Some(Who::Player)).then(|| live_actor(leader.id)).flatten()
                            })
                        } else {
                            None
                        };
                        return (index, choice);
                    }
                }
                if retaliating || mob.ai.target.is_some() {
                    return (index, None);
                }
                let eye = Vec3::new(mob.position.x, mob.eye_y(), mob.position.z);
                let can_hunt = match kind {
                    shark::KIND => {
                        let [x, y, z] = eye.components().map(|coordinate| coordinate.floor() as i32);
                        hunt::cooldown(&mob.ai.nav) <= 0 && world.brightness(x, y, z) < 0.5
                    }
                    PIRANHA => true,
                    ALLIGATOR | hippo::KIND => !mob.is_baby(),
                    _ => false,
                };
                if !can_hunt {
                    return (index, None);
                }
                let water_required = kind != ALLIGATOR || (world.daylight && !mob.extra.flag && !mob.extra.flag2);
                let eligible = |player: &PlayerView| {
                    let range = (mob.attrs.follow_range
                        * player.visibility_percent(Some(mob.kind.id.as_str()), world.items)).max(2.0);
                    player.alive && !player.creative && !player.spectator
                        && !player.allies.contains(&mob.id)
                        && (kind != hippo::KIND || mob.extra.owner != Some(player.uuid))
                        && mob.distance_sqr(player.position) <= range * range
                        && world.line_of_sight_for(
                            Some(mob.position.y), eye,
                            Vec3::new(player.position.x, player.eye[1], player.position.z),
                        )
                };
                let choice = players().filter_map(|(connection, player)| {
                    if !eligible(player) {
                        return None;
                    }
                    // The nest goal outranks an Alligator's ordinary player goal.
                    let priority = if kind == ALLIGATOR && near_eggs(&world, player.position) {
                        0
                    } else if !water_required || world.fluids(player.bounds, player.eye[1]).in_water {
                        1
                    } else {
                        return None;
                    };
                    Some((connection, priority, eye.distance_squared(player.position)))
                }).min_by(|(_, ap, ad), (_, bp, bd)| ap.cmp(bp).then_with(|| ad.total_cmp(bd)))
                    .map(|(connection, _, _)| connection);
                (index, choice)
            })
            .collect::<Vec<_>>();
        (choices, nest_actors)
    };
    for (index, choice) in choices {
        if let Some(mob) = game.mobs.get_mut(index) {
            *mob.ai.nav.__horizon_mixins_naturalist_wet_player_target_mut() = choice;
            if alligator::pursuing_swimmer(mob) {
                alligator::set_nest_actors(mob, nest_actors.clone());
            }
        }
    }
}
