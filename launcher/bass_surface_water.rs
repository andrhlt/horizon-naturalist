fn bass_surface_water(tag: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::spawn_selection::naturalist_aquatic_spawn_limits;
    if tag == "minecraft:allows_tropical_fish_spawns_at_any_height" && naturalist_aquatic_spawn_limits::is_bass() {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
