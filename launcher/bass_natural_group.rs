fn bass_natural_group(game: &Game<'_>, request: &SpawnRequest<'_>, pending: &mut Vec<Mob>) -> std::ops::ControlFlow<Option<Spawned>> {
    use horizon_client::mobs::{
        finalize::{baby_start_age, set_age},
        nav::naturalist_bass_state::{self as bass, NaturalGroup},
        raider,
    };
    use horizon_core::random::RandomSource;
    use std::ops::ControlFlow;

    if request.entity == "naturalist:deer" {
        let finalized = horizon_core::random::naturalist_deer_level_spawn::with_current(|random| {
            let kind = game.res.mob_types().get(request.entity)?.clone();
            let [x, y, z] = request.position;
            let mut mob = Mob::new(0, kind, Vec3::new(x, y, z), request.yaw);
            let variant = super::naturalist_deer_variants::choose_spawn_variant(game, &mob, random)
                .unwrap_or_else(|| request.entity.to_owned());
            *mob.ai.nav.__horizon_mixins_naturalist_deer_spawn_variant_mut() = Some(variant);
            if request.group_index > 0 && random.next_float() <= 0.05 {
                let age = baby_start_age(mob.species());
                set_age(&mut mob, age);
            }
            horizon_client::mobs::naturalist_deer_spawn::finish(&mut mob, random);
            let bounds = mob.bounds();
            pending.push(mob);
            Some(Spawned { bounds, school: true })
        });
        if let Some(spawned) = finalized {
            return ControlFlow::Break(spawned);
        }
    }
    let lion = request.entity == "naturalist:lion";
    let whale = request.entity == "naturalist:whale";
    if whale {
        let [x, y, z] = request.position.map(|coord| coord.floor() as i32);
        let facts = game.res.facts();
        if (y..y + 3).any(|height| facts.get(game.world.block(x, height, z)).fluid != horizon_client::blocks::Fluid::Water) {
            return ControlFlow::Break(None);
        }
    }
    let piranha = request.entity == "naturalist:piranha";
    let blobfish = request.entity == "naturalist:blobfish";
    let land_chance = if request.group_index == 0 { None } else { match request.entity {
        "naturalist:zebra" => Some(0.2),
        "naturalist:rhino" | "naturalist:bear" | "naturalist:black_bear" => Some(1.0),
        "naturalist:deer" | "naturalist:duck" | "naturalist:boar" | "naturalist:turkey"
        | "naturalist:ostrich" | "naturalist:capybara" | "naturalist:lizard" | "naturalist:komodo_dragon" | "naturalist:tiger" => Some(0.05),
        _ => None,
    }};
    if request.entity != "naturalist:bass" && !piranha && !blobfish && !lion && !whale && land_chance.is_none() {
        return ControlFlow::Continue(());
    }
    let Some(kind) = game.res.mob_types().get(request.entity).cloned() else {
        return ControlFlow::Break(None);
    };
    let [x, y, z] = request.position;
    if piranha || blobfish {
        let location = request.position.map(|coord| coord.floor() as i32);
        let [bx, by, bz] = location;
        let inside_border = game.border.geometry().is_within_bounds(f64::from(bx), f64::from(bz), 0.0);
        let max_y = horizon_world::level::sea_level_of(game.level_key())
            - if blobfish { horizon_client::mobs::nav::naturalist_blobfish_state::DEEP_OFFSET } else { 0 };
        if by > max_y || !inside_border
            || game.res.facts().get(game.world.block(bx, by, bz)).fluid != horizon_client::blocks::Fluid::Water
            || game.res.mob_data().state(game.world.block(bx, by + 1, bz)).flags & data::REDSTONE_CONDUCTOR != 0
        {
            return ControlFlow::Break(None);
        }
    }
    if blobfish { return ControlFlow::Continue(()); }
    let mut mob = Mob::new(0, kind, Vec3::new(x, y, z), request.yaw);
    if lion {
        horizon_client::mobs::goals::naturalist_lion_goals::mark_group_member(&mut mob, request.group_index);
        let bounds = mob.bounds();
        pending.push(mob);
        return ControlFlow::Break(Some(Spawned { bounds, school: false }));
    }
    let facts = game.biome_facts(x.floor() as i32, y.floor() as i32, z.floor() as i32);
    let difficulty = game.local_difficulty(request.position);
    horizon_client::mobs::spawning::finalize(&mut mob, &facts, &difficulty);

    if whale {
        if request.group_index > 0 {
            set_age(&mut mob, -24000);
        }
        let bounds = mob.bounds();
        pending.push(mob);
        return ControlFlow::Break(Some(Spawned { bounds, school: true }));
    }
    if let Some(chance) = land_chance {
        raider::finalize(&mut mob, raider::Reason::Natural);
        if mob.random.next_float() <= chance {
            let age = baby_start_age(mob.species());
            set_age(&mut mob, age);
        }
        let bounds = mob.bounds();
        pending.push(mob);
        return ControlFlow::Break(Some(Spawned { bounds, school: true }));
    }
    let index = request.group_index;
    let formation = if index == 0 {
        let variant = super::naturalist_deer_variants::select_group_variant(game, &mut mob)
            .unwrap_or_else(|| request.entity.to_owned());
        let target = if piranha { u32::MAX } else { match bass::tier(&variant) {
            2 => 1,
            1 => 2 + mob.random.next_int_bound(2).unwrap_or(0) as u32,
            _ => 3 + mob.random.next_int_bound(4).unwrap_or(0) as u32,
        } };
        NaturalGroup { variant, target, index, leader_position: request.position }
    } else {
        let Some(mut leader) = pending.len().checked_sub(index as usize)
            .and_then(|leader| pending.get(leader))
            .and_then(|leader| bass::group(&leader.ai.nav)).cloned()
        else {
            return ControlFlow::Break(None);
        };
        if index >= leader.target {
            return ControlFlow::Break(None);
        }
        leader.index = index;
        if piranha {
            leader.variant = super::naturalist_deer_variants::select_group_variant(game, &mut mob)
                .unwrap_or_else(|| request.entity.to_owned());
        }
        leader
    };
    if !piranha { mob.extra.variant = bass::tier(&formation.variant); }
    let school = formation.index + 1 < formation.target;
    bass::set_group(&mut mob.ai.nav, formation);
    let bounds = mob.bounds();
    pending.push(mob);
    ControlFlow::Break(Some(Spawned { bounds, school }))
}
