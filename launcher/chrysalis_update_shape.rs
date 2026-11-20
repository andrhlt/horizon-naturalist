fn chrysalis_update_shape(
    level: &GameLevel<'_>,
    state: BlockStateId,
    direction: Direction,
    neighbour: BlockStateId,
    new: &mut Option<BlockStateId>,
) {
    let Some(row) = level.registry.state(state) else { return };
    if row.name != "naturalist:chrysalis" {
        return;
    }
    let facing = row.properties.get("facing").map(String::as_str);
    let face = match direction {
        Direction::North => "north",
        Direction::East => "east",
        Direction::South => "south",
        Direction::West => "west",
        _ => return,
    };
    if facing == Some(face) {
        let block = level.behaviours.block(neighbour);
        if !level.behaviours.tag("minecraft:logs").is_some_and(|tag| tag.contains(block)) {
            *new = Some(level.world.air);
        }
    }
}
