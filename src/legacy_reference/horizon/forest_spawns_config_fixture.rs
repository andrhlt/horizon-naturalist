fn forest_spawns_config_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;

    if command != "naturalist_fixture_forest_spawns" {
        return ControlFlow::Continue(());
    }
    let pos = [8, 107, 8];
    let name = game.biome_at([8.5, 107.5, 8.5]);
    let catalog = world_biomes!(game);
    let biome = catalog.get(catalog.id(name));
    let mut entries = biome.spawns.get("creature").cloned().unwrap_or_default();
    let mut canceled = false;
    game.native_potential_spawns(game.dim.id(), "creature", pos, &mut entries, &mut canceled);
    let mut rows: Vec<_> = entries.into_iter()
        .filter(|entry| matches!(entry.entity.as_str(), "minecraft:fox" | "minecraft:rabbit"))
        .map(|entry| (entry.entity, entry.weight, entry.min, entry.max))
        .collect();
    rows.sort();
    let json = serde_json::json!({"biome":name, "canceled":canceled, "rows":rows});
    game.feedback(Text::literal(format!("Naturalist forest fixture {json}")));
    ControlFlow::Break(true)
}
