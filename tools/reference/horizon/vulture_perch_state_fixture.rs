fn vulture_perch_state_fixture(
    game: &Game<'_>,
    line: &str,
) -> std::ops::ControlFlow<Result<Option<Value>>> {
    use std::ops::ControlFlow;
    if line != "naturalist_vulture_perch" || !game.options.headless {
        return ControlFlow::Continue(());
    }
    ControlFlow::Break(Ok(Some(game.naturalist_fixture_vulture_perch_state())))
}
