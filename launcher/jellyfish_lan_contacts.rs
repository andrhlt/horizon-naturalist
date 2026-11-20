fn jellyfish_lan_contacts(game: &mut Game<'_>) {
    let Some(lan) = game.lan.as_ref() else { return };
    let fish: Vec<_> = game.mobs.iter().filter_map(|mob| {
        (mob.kind.id == "naturalist:jellyfish" && mob.alive() && !mob.removed
            && game.creatures.ticked.contains(&mob.id)).then(|| {
            (mob.id, mob.bounds().inflate(Vec3::new(0.2, 0.2, 0.2)),
                mob.position.components(), lan.mob_players.get(&mob.id).copied())
        })
    }).collect();
    if fish.is_empty() { return; }

    let host = game.player_view();
    let guests = game.lan_guest_views();
    let mut extra = Vec::new();
    for (id, bounds, origin, primary) in fish {
        // The regular mob event already handles the player chosen for this
        // fish's turn. Jellyfish contact queries every other living player too.
        if primary.is_some() && host.as_ref().is_some_and(|player| {
            player.alive && !player.spectator && player.bounds.intersects(bounds)
        }) {
            game.hurt_player_from(2.0, Some(origin), "minecraft:mob_attack", Some(Some(id)));
        }
        extra.extend(guests.iter().filter_map(|(guest, player)| {
            (Some(*guest) != primary && player.alive && !player.spectator
                && player.bounds.intersects(bounds))
                .then_some((*guest, id, MobEvent::AttackPlayer(2.0, origin)))
        }));
    }
    if extra.is_empty() { return; }
    if let Some(mut lan) = game.lan.take() {
        for (conn, id, event) in extra {
            game.guest_hit_by(&mut lan, conn, |side| side.guest_mob_event(id, event));
        }
        game.lan = Some(lan);
    }
}
