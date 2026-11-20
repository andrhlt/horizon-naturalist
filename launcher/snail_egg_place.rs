fn snail_egg_place(level: &mut GameLevel<'_>, state: BlockStateId, pos: BlockPos) {
    if level.native_authoritative
        && level
            .registry
            .state(state)
            .is_some_and(|row| row.name == "naturalist:snail_eggs")
    {
        level.events.push(LevelEvent::BlockEvent {
            pos,
            state,
            kind: "naturalist:schedule_snail_hatch",
        });
    }
}
