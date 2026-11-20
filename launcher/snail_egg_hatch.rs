fn snail_egg_hatch(game: &mut Game<'_>) {
    let mut hatching = Vec::new();
    let mut schedule = Vec::new();
    game.level_events.retain(|event| match event {
        LevelEvent::BlockEvent {
            pos,
            kind: "naturalist:schedule_snail_hatch",
            ..
        } => {
            schedule.push(*pos);
            false
        }
        LevelEvent::BlockEvent {
            pos,
            kind: "naturalist:snail_hatch",
            ..
        } => {
            hatching.push(*pos);
            false
        }
        _ => true,
    });
    for pos in schedule {
        if game
            .res
            .facts()
            .get(game.world.block(pos.x, pos.y, pos.z))
            .name
            .as_ref()
            != "naturalist:snail_eggs"
        {
            continue;
        }
        let delay = 600 + game.world.random.next_int_bound(1800).unwrap_or(0);
        game.with_level(|behaviour, level, ticks| {
            behaviour.schedule_tick(level, ticks, pos, delay)
        });
    }
    let edge = f64::from(0.4_f32 / 2.0);
    for pos in hatching {
        let count = 2 + game.world.random.next_int_bound(4).unwrap_or(0);
        for _ in 0..count {
            let x = game.world.random.next_double().clamp(edge, 1.0 - edge);
            let z = game.world.random.next_double().clamp(edge, 1.0 - edge);
            let yaw = 1 + game.world.random.next_int_bound(360).unwrap_or(0);
            let at = [f64::from(pos.x) + x, f64::from(pos.y), f64::from(pos.z) + z];
            if let Some(id) = game.spawn_mob_raw("naturalist:snail", at, true)
                && let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id)
            {
                horizon_client::mobs::finalize::set_age(mob, -6000);
                mob.yaw = yaw as f32;
                mob.previous_yaw = mob.yaw;
                mob.body_yaw = mob.yaw;
                mob.head_yaw = mob.yaw;
            }
        }
    }
}
