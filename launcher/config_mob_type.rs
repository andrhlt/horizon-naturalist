fn config_mob_type(entity: &str) -> std::ops::ControlFlow<Option<(Species, f32, f32, Category)>> {
    if horizon_client::mobs::naturalist_config::removed(entity) {
        std::ops::ControlFlow::Break(None)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
