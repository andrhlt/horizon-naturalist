fn route_level(game: &mut Game<'_>) {
    if let Some(biomes) = &mut game.world_biomes {
        super::super::naturalist_spawn_routes::route(biomes);
    }
}
