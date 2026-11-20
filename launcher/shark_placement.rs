fn shark_placement(
    game: &Game<'_>,
    request: &SpawnRequest<'_>,
) -> std::ops::ControlFlow<Option<Spawned>> {
    use horizon_client::blocks::Fluid;
    use std::ops::ControlFlow;
    if request.entity != horizon_client::mobs::nav::naturalist_shark_state::KIND {
        return ControlFlow::Continue(());
    }
    let [x, y, z] = request.position.map(|part| part.floor() as i32);
    let facts = game.res.facts();
    if [y, y + 1]
        .into_iter()
        .any(|height| facts.get(game.world.block(x, height, z)).fluid != Fluid::Water)
    {
        return ControlFlow::Break(None);
    }
    ControlFlow::Continue(())
}
