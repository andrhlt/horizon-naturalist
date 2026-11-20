fn ant_hill_random_tick(
    level: &mut GameLevel<'_>,
    ticks: &mut Ticks,
    pos: BlockPos,
    state: BlockStateId,
    handled: &mut bool,
) {
    use super::naturalist_ant_hill::RELEASE_EVENTS;
    use horizon_core::{geometry::Direction, random::RandomSource};
    let Some(block) = level.registry.state(state) else {
        return;
    };
    if block.name != super::naturalist_ant_hill::HILL {
        return;
    }
    *handled = true;
    let workers = block
        .properties
        .get("workers")
        .and_then(|value| value.parse::<u8>().ok())
        .unwrap_or(0);
    if workers == 0
        || block.properties.get("open").map(String::as_str) != Some("true")
        || level.world.random.next_float() >= 0.07
    {
        return;
    }

    let directions: Vec<_> = Direction::ALL
        .into_iter()
        .enumerate()
        .filter(|(_, direction)| {
            let [dx, dy, dz] = direction.normal();
            let [x, y, z] = [pos.x + dx, pos.y + dy, pos.z + dz];
            let is_air = |y| level.facts.is_air(level.world.block(x, y, z));
            is_air(y) && !is_air(y - 1)
        })
        .collect();
    if directions.is_empty() {
        return;
    }
    let roll = level
        .world
        .random
        .next_int_bound(directions.len() as i32)
        .expect("nonempty release positions") as usize;
    if level.native_set_block_property(
        ticks,
        [pos.x, pos.y, pos.z],
        "workers",
        &(workers - 1).to_string(),
        3,
    ) {
        level.events.push(horizon_world::level::LevelEvent::BlockEvent {
            pos,
            state,
            kind: RELEASE_EVENTS[directions[roll].0],
        });
    }
}
