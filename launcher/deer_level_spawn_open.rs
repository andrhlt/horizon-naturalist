fn deer_level_spawn_open(mut random: LevelRandom, game: &Game<'_>) -> LevelRandom {
    if game.world.game_time % 400 == 0 && game.ui.rules.bool("spawn_mobs") {
        horizon_core::random::naturalist_deer_level_spawn::open(&mut random);
    }
    random
}
