fn ant_food_targets(game: &mut Game<'_>) {
    if game.remote.is_none() {
        super::naturalist_ant_workers::update_food_targets(game);
    }
}
