fn ant_mob_actions(game: &mut Game<'_>) {
    if game.remote.is_none() {
        super::naturalist_ant_workers::apply_actions(game);
    }
}
