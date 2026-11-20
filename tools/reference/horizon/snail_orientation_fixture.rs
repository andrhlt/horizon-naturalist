fn snail_orientation_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if command != "naturalist_fixture_snail_orientation" || !game.options.headless {
        return ControlFlow::Continue(());
    }
    let frames = game.naturalist_fixture_snail_orientation();
    game.feedback(Text::literal(format!(
        "Snail orientation fixture wrote {frames} frames"
    )));
    ControlFlow::Break(true)
}
