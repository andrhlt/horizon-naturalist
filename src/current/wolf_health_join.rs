fn wolf_health_join(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    crate::game::naturalist_wolf_health::restore(game, entity, kind, None);
}
