fn snail_shell_rotation(
    level: &GameLevel<'_>,
    state: BlockStateId,
    context: &PlacementContext,
    answer: &mut Option<Option<BlockStateId>>,
) {
    use horizon_client::mobs::goals::naturalist_insect_goals::state_property;
    if level
        .registry
        .state(state)
        .is_some_and(|row| row.name == "naturalist:snail_shell")
    {
        let rotation = ((context.yaw * (16.0 / 360.0) + 0.5).floor() as i32).rem_euclid(16);
        *answer = Some(state_property(
            level.registry,
            state,
            "rotation",
            &rotation.to_string(),
        ));
    }
}
