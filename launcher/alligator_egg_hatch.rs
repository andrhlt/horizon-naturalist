fn alligator_egg_hatch(game: &mut Game<'_>) {
    let mut hatching = Vec::new();
    game.level_events.retain(|event| match event {
        LevelEvent::BlockEvent {
            pos,
            state,
            kind,
        } if matches!(*kind, "naturalist:alligator_hatch" | "naturalist:tortoise_hatch" | "naturalist:ostrich_hatch" | "naturalist:tortoise_mined_hatch") => {
            hatching.push((*pos, *state, *kind));
            false
        }
        _ => true,
    });
    use horizon_client::mobs::goals::naturalist_tortoise_goals as tortoise;
    for (pos, state, event) in hatching {
        if event == "naturalist:tortoise_mined_hatch" {
            let current = game.world.block(pos.x, pos.y, pos.z);
            if current == state {
                continue;
            }
            if current != game.world.air {
                game.with_level(|behaviour, level, ticks| {
                    use horizon_world::level::flags::{UPDATE_CLIENTS, UPDATE_NEIGHBORS};
                    behaviour.set_block(level, ticks, pos, level.world.air, UPDATE_CLIENTS | UPDATE_NEIGHBORS);
                });
            }
            use horizon_core::random::RandomSource;
            game.level_events.push(LevelEvent::Sound {
                pos, sound: "naturalist:entity.tortoise.egg_hatch".into(), volume: 0.7,
                pitch: 0.9 + game.world.random.next_float() * 0.2,
            });
            game.level_events.push(LevelEvent::LevelEvent { id: 2001, pos, data: state.0 as i32 });
        }
        let Some(row) = game.pack.assets.registry.state(state) else { continue };
        let kind = match event {
            "naturalist:alligator_hatch" => "naturalist:alligator",
            "naturalist:ostrich_hatch" => "naturalist:ostrich",
            _ => tortoise::KIND,
        };
        let count = if kind == "naturalist:ostrich" { 1 }
            else { row.properties.get("eggs").and_then(|count| count.parse::<u8>().ok()).unwrap_or(1) };
        let variant = row.properties.get("variant").and_then(|n| n.parse::<usize>().ok())
            .and_then(|n| tortoise::COLORS.get(n).copied()).unwrap_or(tortoise::COLORS[0]);
        for j in 0..count {
            let at = [
                f64::from(pos.x) + 0.3 + f64::from(j) * 0.2,
                f64::from(pos.y),
                f64::from(pos.z) + 0.3,
            ];
            if let Some(id) = game.spawn_mob_raw(kind, at, false) {
                if kind == tortoise::KIND {
                    crate::game::naturalist_deer_variants::put_variant(game, id, variant);
                }
                let Some(mob) = game.mobs.iter_mut().find(|m| m.id == id) else { continue };
                horizon_client::mobs::finalize::set_age(mob, -24000);
                mob.yaw = 0.0;
                mob.previous_yaw = 0.0;
                mob.body_yaw = 0.0;
                mob.head_yaw = 0.0;
            }
        }
    }
}
