fn shark_wet_player_candidates(_result: (), game: &mut Game<'_>) {
    use horizon_client::mobs::{
        PlayerView, Who, nav::{naturalist_catfish_state as hunt, naturalist_shark_state as shark},
    };
    use horizon_core::geometry::Vec3;

    let sharks = game.mobs.iter().any(|mob| mob.kind.id == shark::KIND && mob.alive());
    if !sharks {
        return;
    }
    let guests = game.lan_guest_views();
    let host = game.player_view();
    let choices = {
        let world = game.mob_world_base(&[]);
        let players = || {
            host.iter().map(|player| (None, player))
                .chain(guests.iter().map(|(conn, player)| (Some(*conn), player)))
        };
        game.mobs.iter().enumerate()
            .filter(|(_, mob)| mob.kind.id == shark::KIND && mob.alive())
            .map(|(index, mob)| {
                let already_bound = mob.ai.target.is_some()
                    || (mob.ai.last_hurt_by == Some(Who::Player)
                        && mob.age as i32 - mob.ai.last_hurt_by_time <= 100)
                    || mob.extra.leash.is_some();
                let eye = Vec3::new(mob.position.x, mob.eye_y(), mob.position.z);
                let [x, y, z] = eye.components().map(|coordinate| coordinate.floor() as i32);
                if guests.is_empty() || mob.no_ai || already_bound || hunt::cooldown(&mob.ai.nav) > 0
                    || world.brightness(x, y, z) >= 0.5
                {
                    return (index, None);
                }
                let eligible = |player: &PlayerView| {
                    let range = (mob.attrs.follow_range
                        * player.visibility_percent(Some(mob.kind.id.as_str()), world.items)).max(2.0);
                    player.alive && !player.creative && !player.spectator
                        && !player.allies.contains(&mob.id)
                        && mob.distance_sqr(player.position) <= range * range
                        && world.fluids(player.bounds, player.eye[1]).in_water
                        && world.line_of_sight_for(
                            Some(mob.position.y), eye,
                            Vec3::new(player.position.x, player.eye[1], player.position.z),
                        )
                };
                let choice = players()
                    .filter(|(_, player)| eligible(player))
                    .min_by(|(_, a), (_, b)| eye.distance_squared(a.position)
                        .total_cmp(&eye.distance_squared(b.position)))
                    .map(|(connection, _)| connection);
                (index, choice)
            })
            .collect::<Vec<_>>()
    };
    for (index, choice) in choices {
        if let Some(mob) = game.mobs.get_mut(index) {
            shark::motion_mut(&mut mob.ai.nav).wet_player = choice;
        }
    }
}
