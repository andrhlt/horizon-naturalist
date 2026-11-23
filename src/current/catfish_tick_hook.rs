fn catfish_tick(game: &mut Game<'_>, entity: u32, kind: &str) {
    crate::game::naturalist_catfish::tick(game, entity, kind)
}
