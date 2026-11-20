fn config_spawn_entries(
    game: &Game<'_>, category: &str, pos: [i32; 3], entries: &mut Vec<SpawnEntry>,
) {
    use horizon_client::mobs::naturalist_config::removed;
    use super::naturalist_deer_variants::biome_matches;

    entries.retain(|entry| !removed(&entry.entity));
    if category != "creature" {
        return;
    }
    let disable_fox = removed("naturalist:forest_fox");
    let disable_rabbit = removed("naturalist:forest_rabbit");
    if !disable_fox && !disable_rabbit {
        return;
    }
    let biome = game.biome_at(pos.map(|n| f64::from(n) + 0.5));
    if !biome_matches(game, "#minecraft:is_forest", biome) {
        return;
    }
    for (disabled, species, blacklist, min, max) in [
        (disable_fox, "minecraft:fox", "#naturalist:blacklist/blacklist_forest_foxes", 1, 2),
        (disable_rabbit, "minecraft:rabbit", "#naturalist:blacklist/blacklist_forest_rabbits", 2, 3),
    ] {
        if disabled && !biome_matches(game, blacklist, biome)
            && let Some(index) = entries.iter().rposition(|entry| {
                entry.entity == species && entry.weight == 10 && entry.min == min && entry.max == max
            })
        {
            entries.remove(index);
        }
    }
}
