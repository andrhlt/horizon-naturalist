fn shark_animation_tick(game: &mut Game<'_>) {
    game.tick_naturalist_shark_animations();
    game.tick_naturalist_whale_animations();
}
