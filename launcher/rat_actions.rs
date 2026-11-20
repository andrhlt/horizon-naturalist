fn rat_actions(game: &mut Game<'_>) {
    if game.remote.is_none() {
        super::naturalist_rat_work::actions(game);
    }
}
