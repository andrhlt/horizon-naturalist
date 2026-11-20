fn crab_snapshot(game: &mut Game<'_>) {
    use horizon_client::mobs::nav::naturalist_crab as crab;

    if !game.mobs.iter().any(|mob| mob.kind.id == crab::KIND) {
        return;
    }
    let guests = game.lan_guest_views();
    let world = game.mob_world_base(&[]);
    let items = game.items();
    let changes: Vec<_> = game.mobs.iter().enumerate().filter(|(_, mob)| mob.kind.id == crab::KIND)
        .map(|(index, mob)| {
            let old = crab::state(&mob.ai.nav);
            let area = mob.bounds().inflate(Vec3::new(8.0, 8.0, 8.0));
            let mut nearest = None;
            let mut weapon = None;
            if !crab::holding_weapon(mob) {
                for item in game.dropped.iter().filter(|item| item.pickup_delay <= 0
                    && area.intersects(item.bounds())
                    && crab::WEAPON_TAGS.iter().any(|tag| items.item_tags.get(*tag).is_some_and(|values| values.contains(item.stack.item.as_ref()))))
                {
                    let found = crab::Weapon { id: item.id, at: item.position.components() };
                    let distance = mob.distance_sqr(item.position);
                    if nearest.as_ref().is_none_or(|(_, best)| distance < *best) {
                        nearest = Some((found, distance));
                    }
                    if old.target == Some(found.id) { weapon = Some(found); }
                }
            }
            let near = world.player.iter().chain(guests.iter().map(|(_, player)| player))
                .any(|player| crab::visible(mob, player, &world));
            let dancing = if (mob.age + 1) % 20 == 0 {
                let [x, y, z] = mob.position.components().map(|coordinate| coordinate.floor() as i32);
                mob.extra.tame && !mob.extra.sitting && (-3..=3).any(|dx|
                    (-1..=1).any(|dy| (-3..=3).any(|dz| {
                        let state = game.world.block(x + dx, y + dy, z + dz);
                        game.pack.assets.registry.state(state).is_some_and(|state| {
                            state.name == "minecraft:jukebox"
                                && state.properties.get("has_record").is_some_and(|value| value == "true")
                        })
                    })))
            } else {
                old.dancing
            };
            (index, near, nearest.map(|(weapon, _)| weapon), weapon, dancing)
        }).collect();
    for (index, near, nearest, weapon, dancing) in changes {
        let state = crab::state_mut(&mut game.mobs[index].ai.nav);
        state.player_near = near;
        state.nearest = nearest;
        state.weapon = weapon;
        state.dancing = dancing;
    }
}
