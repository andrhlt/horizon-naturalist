fn insect_spawn_command(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if command != "naturalist_fixture_insect_spawn" || !game.options.headless {
        return ControlFlow::Continue(());
    }
    let result = game.naturalist_fixture_insect_spawn();
    game.feedback(Text::literal(format!("Insect spawn fixture {result}")));
    ControlFlow::Break(true)
}
