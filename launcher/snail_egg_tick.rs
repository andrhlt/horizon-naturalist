fn snail_egg_tick(
    level: &mut GameLevel<'_>,
    ticks: &mut Ticks,
    pos: BlockPos,
    state: BlockStateId,
    handled: &mut bool,
) {
    use horizon_world::level::flags::{UPDATE_CLIENTS, UPDATE_NEIGHBORS};
    if !level
        .registry
        .state(state)
        .is_some_and(|row| row.name == "naturalist:snail_eggs")
    {
        return;
    }
    *handled = true;
    let behaviour = level.behaviours;
    if behaviour.set_block(
        level,
        ticks,
        pos,
        level.world.air,
        UPDATE_NEIGHBORS | UPDATE_CLIENTS,
    ) {
        level.events.push(LevelEvent::Sound {
            pos,
            sound: "minecraft:block.frogspawn.hatch".into(),
            volume: 1.0,
            pitch: 1.0,
        });
        level.events.push(LevelEvent::LevelEvent {
            id: 2001,
            pos,
            data: state.0 as i32,
        });
        level.events.push(LevelEvent::BlockEvent {
            pos,
            state,
            kind: "naturalist:snail_hatch",
        });
    }
}
