fn bass_natural_fixture_command(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if command != "naturalist_fixture_bass_natural" {
        return ControlFlow::Continue(());
    }
    let result = game.naturalist_fixture_bass_natural();
    game.feedback(Text::literal(format!("Bass natural fixture {result}")));
    ControlFlow::Break(true)
}
