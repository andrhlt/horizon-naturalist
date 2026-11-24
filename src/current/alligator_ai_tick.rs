fn alligator_ai_tick(game: &mut Game<'_>, entity: u32, kind: &str) {
    if kind == "naturalist:alligator" {
        crate::game::naturalist_alligator_goals::install(game, entity);
    }
}
