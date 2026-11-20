fn snail_crush(game: &mut Game<'_>) {
    use horizon_client::mobs::naturalist_config;
    use horizon_core::geometry::{Aabb, Vec3};

    if game.remote.is_some() || !naturalist_config::snail_crushing() {
        return;
    }
    let feather_falling = |boots: &Option<horizon_client::items::ItemStack>| {
        boots
            .as_ref()
            .is_some_and(|boots| boots.enchantment_level("minecraft:feather_falling") > 0)
    };
    let local = game
        .player
        .as_ref()
        .filter(|player| {
            !player.ground.on_ground && !game.spectator() && !feather_falling(&game.armor[3])
        })
        .map(|player| {
            let at = player.position;
            let half_width = f64::from(player.dimensions.width) / 2.0;
            let height = f64::from(player.dimensions.height);
            let bounds = Aabb::new(
                at.add_vector(Vec3::new(-half_width, 0.0, -half_width)),
                at.add_vector(Vec3::new(half_width, height, half_width)),
            );
            (player.entity_id as u32, bounds)
        });
    let guests = game.lan.iter().flat_map(|host| {
        host.hub
            .guests
            .iter()
            .filter(|player| {
                host.hub.here(player)
                    && !player.on_ground
                    && !player.spectator()
                    && !feather_falling(&player.armor[3])
            })
            .map(|player| (player.entity_id as u32, player.bounds()))
    });
    let players: Vec<_> = local.into_iter().chain(guests).collect();
    if players.is_empty() {
        return;
    }

    let ticking = game.entity_ticking_area();
    let contacts: Vec<_> = game
        .mobs
        .iter()
        .filter_map(|snail| {
            if snail.kind.id != "naturalist:snail"
                || !snail.alive()
                || snail.removed
                || !snail.on_ground
                || snail.custom_name.is_some()
                || snail.persistent
                || !ticking(
                    snail.position.x.floor() as i32,
                    snail.position.z.floor() as i32,
                )
            {
                return None;
            }
            if game
                .persist
                .raw
                .get(&snail.id)
                .is_some_and(|tag| horizon_client::entity_nbt::flag(tag, "FromHand"))
            {
                return None;
            }
            let bounds = snail.bounds().inflate(Vec3::new(0.2, 0.5, 0.2));
            let (player, _) = players.iter().find(|(_, player)| {
                player.min_position().y > snail.position.y && bounds.intersects(*player)
            })?;
            Some((snail.id, snail.attrs.max_health * 2.0, *player))
        })
        .collect();
    let level = game.dim.id();
    for (snail, damage, player) in contacts {
        game.native_hurt_entity(
            level,
            snail,
            damage,
            "minecraft:player_attack",
            Some(player),
        );
    }
}
