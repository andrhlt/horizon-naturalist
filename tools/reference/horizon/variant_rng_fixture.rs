fn variant_rng_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    let Some(seed) = command.strip_prefix("naturalist_fixture_variant_rng ") else {
        return ControlFlow::Continue(());
    };
    if !game.options.headless {
        return ControlFlow::Continue(());
    }
    let Ok(seed) = seed.parse::<i64>() else { return ControlFlow::Break(false) };
    let result = super::naturalist_deer_variants::fixture_rng(game, seed);
    game.feedback(Text::literal(format!("Naturalist variant RNG {result}")));
    ControlFlow::Break(true)
}
