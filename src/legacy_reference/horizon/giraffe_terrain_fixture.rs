fn giraffe_terrain_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if command != "naturalist_fixture_giraffe_terrain" || !game.options.headless {
        return ControlFlow::Continue(());
    }
    let Some(report) = game.naturalist_fixture_giraffe_terrain() else {
        return ControlFlow::Break(false);
    };
    game.feedback(Text::literal(format!(
        "Naturalist Giraffe terrain {report}"
    )));
    ControlFlow::Break(true)
}
