fn plush_bear_placement(
    level: &GameLevel<'_>,
    state: BlockStateId,
    context: &PlacementContext,
    answer: &mut Option<Option<BlockStateId>>,
) {
    use horizon_client::mobs::goals::naturalist_insect_goals::state_property;

    if answer.is_some()
        || !level.registry.state(state).is_some_and(|row| row.name == "naturalist:plush_bear")
    {
        return;
    }
    // Direction.fromYRot(yaw).opposite(), the bear faces its placer.
    let facing = ((f64::from(context.yaw) / 90.0 + 0.5).floor() as i64 & 3) as usize;
    *answer = Some(state_property(level.registry, state, "facing", ["north", "east", "south", "west"][facing]));
}
