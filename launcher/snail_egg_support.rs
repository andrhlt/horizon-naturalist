fn snail_egg_support(
    level: &GameLevel<'_>,
    state: BlockStateId,
    direction: Direction,
    neighbor: BlockStateId,
    new: &mut Option<BlockStateId>,
) {
    if direction == Direction::Down
        && level
            .registry
            .state(state)
            .is_some_and(|row| row.name == "naturalist:snail_eggs")
        && !level.behaviours.is_face_sturdy(neighbor, Direction::Up)
    {
        *new = Some(level.world.air);
    }
}
