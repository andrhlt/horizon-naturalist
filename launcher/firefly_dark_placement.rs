fn firefly_dark_placement(
    level: &GameLevel<'_, '_>,
    entity: &str,
) -> std::ops::ControlFlow<Option<(Species, f32, f32, Category)>> {
    if entity != "naturalist:firefly" {
        return std::ops::ControlFlow::Continue(());
    }
    if horizon_client::mobs::naturalist_config::removed(entity) {
        return std::ops::ControlFlow::Break(None);
    }
    // The placement query only sees the template Species. Use the stock
    // Monster darkness predicate here, while keeping the type's ambient
    // category and its actual entity ID, size and factory.
    std::ops::ControlFlow::Break(level.game.res.mob_types().get(entity).map(|mob| {
        (
            Species::Enderman,
            mob.width as f32,
            mob.height as f32,
            Category::from_name(&mob.category),
        )
    }))
}
