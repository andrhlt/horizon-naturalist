fn deer_spawner_rng_after_add(game: &mut Game<'_>) {
    use horizon_client::text::Text;
    use horizon_core::random::RandomSource;
    let next = game.world.random.clone().next_long();
    game.feedback(Text::literal(format!("Spawner random after-add {next}")));
}
