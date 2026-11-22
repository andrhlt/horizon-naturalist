fn ant_worker_pre(game: &mut Game<'_>, entity: u32, kind: &str) {
    if kind == "naturalist:ant" {
        crate::game::naturalist_ant_worker::prepare(game, entity);
    }
    if kind == "minecraft:item" {
        crate::game::naturalist_ant_worker::follow(game, entity);
    }
}
