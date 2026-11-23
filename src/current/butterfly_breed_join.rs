fn butterfly_breed_join(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    if kind == "naturalist:butterfly" { crate::game::naturalist_butterfly_breed::install(game, entity); }
}
