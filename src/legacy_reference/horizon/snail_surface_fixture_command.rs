fn snail_surface_fixture_command(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if command != "naturalist_fixture_snail_surface" || !game.options.headless {
        return ControlFlow::Continue(());
    }
    let Some(report) = game.naturalist_fixture_snail_surface() else {
        return ControlFlow::Break(false);
    };
    game.feedback(Text::literal(format!("Naturalist snail surface {report}")));
    ControlFlow::Break(true)
}
