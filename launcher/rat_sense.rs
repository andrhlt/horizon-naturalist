fn rat_sense(game: &mut Game<'_>) {
    if game.remote.is_none() {
        super::naturalist_rat_work::sense(game);
    }
}
