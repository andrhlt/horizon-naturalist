fn ant_carried_items(game: &mut Game<'_>) {
    if game.remote.is_none() {
        super::naturalist_ant_workers::update_carried_items(game);
    }
}
