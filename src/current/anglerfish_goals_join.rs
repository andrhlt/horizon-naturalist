fn anglerfish_goals_join(game: &mut Game<'_>, entity: u32, kind: &str) {
    if kind == "naturalist:anglerfish" {
        crate::game::naturalist_anglerfish_goals::install(game, entity);
    }
}
