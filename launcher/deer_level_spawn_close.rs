fn deer_level_spawn_close(game: &mut Game<'_>) {
    horizon_core::random::naturalist_deer_level_spawn::close(&mut game.world.random);
}
