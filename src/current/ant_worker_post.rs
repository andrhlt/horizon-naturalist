fn ant_worker_post(game: &mut Game<'_>, entity: u32, kind: &str) {
    if kind == "naturalist:ant" {
        crate::game::naturalist_ant_worker::finish(game, entity);
    }
}
