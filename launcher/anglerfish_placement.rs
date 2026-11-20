fn anglerfish_placement(game: &Game<'_>, request: &SpawnRequest<'_>) -> std::ops::ControlFlow<Option<Spawned>> {
    use std::ops::ControlFlow;
    if request.entity == "naturalist:anglerfish"
        && !anglerfish_spawn::placement(game, request.position.map(|coordinate| coordinate.floor() as i32))
    {
        return ControlFlow::Break(None);
    }
    ControlFlow::Continue(())
}
