fn chrysalis_random_tick(
    level: &mut GameLevel<'_>,
    ticks: &mut Ticks,
    pos: BlockPos,
    state: BlockStateId,
    handled: &mut bool,
) {
    use horizon_client::mobs::goals::naturalist_insect_goals::state_property;
    use horizon_world::level::flags::{UPDATE_CLIENTS, UPDATE_NEIGHBORS};
    let Some(row) = level.registry.state(state) else { return };
    if row.name != "naturalist:chrysalis" {
        return;
    }
    *handled = true;
    let age = row.properties.get("age").and_then(|v| v.parse::<u8>().ok()).unwrap_or(0);
    let behaviour = level.behaviours;
    if age < 3 {
        if level.world.random.next_int_bound(5) == Ok(0)
            && let Some(next) = state_property(level.registry, state, "age", &(age + 1).to_string())
        {
            behaviour.set_block(level, ticks, pos, next, UPDATE_CLIENTS);
        }
        return;
    }
    if behaviour.set_block(level, ticks, pos, level.world.air, UPDATE_NEIGHBORS | UPDATE_CLIENTS) {
        level.events.push(LevelEvent::Sound {
            pos,
            sound: "minecraft:block.wood.break".into(),
            volume: 0.7,
            pitch: 0.9 + level.world.random.next_float() * 0.2,
        });
        level.events.push(LevelEvent::LevelEvent { id: 2001, pos, data: state.0 as i32 });
        level.events.push(LevelEvent::BlockEvent { pos, state, kind: "naturalist:chrysalis_hatch" });
    }
}
