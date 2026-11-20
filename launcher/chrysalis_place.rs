fn chrysalis_place(
    level: &GameLevel<'_>,
    state: BlockStateId,
    context: &PlacementContext,
    answer: &mut Option<Option<BlockStateId>>,
) {
    use horizon_client::mobs::goals::naturalist_insect_goals::state_property;
    let Some(row) = level.registry.state(state) else {
        return;
    };
    if row.name.as_str() != "naturalist:chrysalis" {
        return;
    }
    let support = level.behaviours.tag("minecraft:logs");
    *answer = Some(
        context
            .naturalist_placement_directions()
            .into_iter()
            .find_map(|direction| {
                let facing = match direction {
                    Direction::North => "north",
                    Direction::East => "east",
                    Direction::South => "south",
                    Direction::West => "west",
                    _ => return None,
                };
                let [x, y, z] = direction.normal();
                let p = context.pos;
                let adjacent = level.world.block(p.x + x, p.y + y, p.z + z);
                if support.is_some_and(|tag| tag.contains(level.behaviours.block(adjacent))) {
                    state_property(level.registry, state, "facing", facing)
                } else {
                    None
                }
            }),
    );
}
