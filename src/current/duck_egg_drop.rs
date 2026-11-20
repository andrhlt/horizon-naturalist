fn duck_egg_drop(
    game: &mut crate::game::Game<'_>,
    mob: u32,
    table: &'static str,
    at: [f64; 3],
) -> std::ops::ControlFlow<bool> {
    if table != "minecraft:gameplay/chicken_lay"
        || !game
            .mobs
            .iter()
            .any(|entry| entry.id == mob && entry.kind.id == "naturalist:duck")
    {
        return std::ops::ControlFlow::Continue(());
    }
    game.spawn_at_location(
        horizon_core::geometry::Vec3::new(at[0], at[1], at[2]),
        0.0,
        horizon_client::items::ItemStack::new("naturalist:duck_egg", 1),
    );
    std::ops::ControlFlow::Break(true)
}
