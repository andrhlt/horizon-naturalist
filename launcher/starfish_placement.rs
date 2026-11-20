fn starfish_placement(game: &Game<'_>, request: &SpawnRequest<'_>) -> std::ops::ControlFlow<Option<Spawned>> {
    use horizon_core::geometry::Direction;

    if !matches!(request.entity, "naturalist:starfish" | "naturalist:giant_isopod" | "naturalist:clam") {
        return std::ops::ControlFlow::Continue(());
    }
    let [x, y, z] = request.position.map(|coordinate| coordinate.floor() as i32);
    let facts = game.res.facts();
    if y >= horizon_world::level::sea_level_of(game.level_key())
        || facts.get(game.world.block(x, y, z)).fluid != horizon_client::blocks::Fluid::Water
        || facts.get(game.world.block(x, y - 1, z)).sturdy_full & (1 << Direction::Up as u8) == 0
    {
        return std::ops::ControlFlow::Break(None);
    }
    std::ops::ControlFlow::Continue(())
}
