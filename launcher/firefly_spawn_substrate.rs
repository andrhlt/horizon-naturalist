fn firefly_spawn_substrate(
    game: &Game<'_>,
    request: &SpawnRequest<'_>,
) -> std::ops::ControlFlow<Option<Spawned>> {
    if request.entity == "naturalist:firefly" {
        let [x, y, z] = request.position.map(|value| value.floor() as i32);
        let level = game.natural_level();
        let below = level.w.block_name(x, y - 1, z);
        if !level
            .w
            .block_tag("naturalist:fireflies_spawnable_on", below)
            || !spawn_selection::naturalist_firefly_walk_target::animal(&level, x, y, z)
        {
            return std::ops::ControlFlow::Break(None);
        }
    }
    std::ops::ControlFlow::Continue(())
}
