fn ant_hill_break(_result: (), game: &mut Game<'_>, pos: [i32; 3], state: BlockStateId, cancelled: &mut bool) {
    if !*cancelled {
        super::naturalist_ant_hill::before_player_break(game, pos, state);
    }
}
