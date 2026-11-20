fn deer_placement_entity(entity: &str) {
    horizon_client::mobs::naturalist_spawn_variants::select_generation_bird(entity);
    super::naturalist_deer_placement::select(entity);
    horizon_client::mobs::spawn_selection::naturalist_aquatic_spawn_limits::select(entity);
}
