fn butterfly_group_command(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if command != "naturalist_fixture_butterfly_group" || !game.options.headless {
        return ControlFlow::Continue(());
    }
    let result = game.naturalist_fixture_butterfly_group();
    game.feedback(Text::literal(format!("Butterfly group fixture {result}")));
    ControlFlow::Break(true)
}
