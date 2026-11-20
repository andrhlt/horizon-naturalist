fn ant_hill_defend(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    super::naturalist_ant_hill::check_defense(game, pos, entity);
}
