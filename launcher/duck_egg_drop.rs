fn duck_egg_drop(game: &mut Game<'_>, mob: u32, table: &str, at: [f64; 3]) -> std::ops::ControlFlow<bool> {
    if table != "minecraft:gameplay/chicken_lay"
        || !game.mobs.iter().any(|entry| entry.id == mob && entry.kind.id == "naturalist:duck")
    {
        return std::ops::ControlFlow::Continue(());
    }

    game.spawn_at_location(Vec3::new(at[0], at[1], at[2]), 0.0, ItemStack::new("naturalist:duck_egg", 1));
    std::ops::ControlFlow::Break(true)
}
