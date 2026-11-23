fn zebra_breed_tick(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    crate::game::naturalist_zebra_breed::install(game, entity, kind);
}
