fn ant_hill_goal_join(game: &mut Game<'_>, entity: u32, kind: &str) {
    if kind == "naturalist:ant" {
        crate::game::naturalist_ant_hill_goal::install(game, entity);
    }
}
