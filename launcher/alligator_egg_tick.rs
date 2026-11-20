fn alligator_egg_tick(
    level: &mut GameLevel<'_>,
    ticks: &mut Ticks,
    pos: BlockPos,
    state: BlockStateId,
    handled: &mut bool,
) {
    use horizon_client::mobs::goals::naturalist_insect_goals::state_property;
    use horizon_world::level::flags::{UPDATE_CLIENTS, UPDATE_NEIGHBORS};

    let Some(row) = level.registry.state(state) else {
        return;
    };
    let ostrich = row.name == "naturalist:ostrich_egg";
    let (crack, hatch, event) = match row.name.as_str() {
        "naturalist:ostrich_egg" => ("naturalist:entity.ostrich.egg_crack", "naturalist:entity.ostrich.egg_hatch", "naturalist:ostrich_hatch"),
        "naturalist:alligator_egg" => ("naturalist:entity.alligator.egg_crack", "naturalist:entity.alligator.egg_hatch", "naturalist:alligator_hatch"),
        "naturalist:tortoise_egg" => ("naturalist:entity.tortoise.egg_crack", "naturalist:entity.tortoise.egg_hatch", "naturalist:tortoise_hatch"),
        _ => return,
    };
    *handled = true;
    if ostrich {
        if level.world.random.next_int_bound(3) != Ok(0) {
            return;
        }
    } else {
        let day = level.world.day_time.rem_euclid(24000);
        let chance =
            if level.dimension_type().has_timeline("minecraft:day") && (21062..21905).contains(&day) {
                1.0
            } else {
                0.002
            };
        if level.world.random.next_float() >= chance {
            return;
        }
    }

    let stage = row
        .properties
        .get("hatch")
        .and_then(|v| v.parse::<u8>().ok())
        .unwrap_or(0);
    let eggs = if ostrich { 1 } else { row
        .properties
        .get("eggs")
        .and_then(|v| v.parse::<u8>().ok())
        .unwrap_or(1) };
    let pitch = 0.9 + level.world.random.next_float() * 0.2;
    let behaviour = level.behaviours;
    let sound = if stage < 2 { crack } else { hatch };
    level.events.push(LevelEvent::Sound {
        pos,
        sound: sound.into(),
        volume: 0.7,
        pitch,
    });
    if stage < 2 {
        if let Some(next) = state_property(level.registry, state, "hatch", &(stage + 1).to_string())
        {
            behaviour.set_block(level, ticks, pos, next, UPDATE_CLIENTS);
        }
    } else if behaviour.set_block(
        level,
        ticks,
        pos,
        level.world.air,
        UPDATE_NEIGHBORS | UPDATE_CLIENTS,
    ) {
        for _ in 0..eggs {
            level.events.push(LevelEvent::LevelEvent {
                id: 2001,
                pos,
                data: state.0 as i32,
            });
        }
        level.events.push(LevelEvent::BlockEvent {
            pos,
            state,
            kind: event,
        });
    }
}
