fn bird_pose_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if command != "naturalist_fixture_bird_pose" || !game.options.headless {
        return ControlFlow::Continue(());
    }
    let frames = game.naturalist_fixture_bird_pose();
    game.feedback(Text::literal(format!(
        "Bird pose fixture wrote {frames} frames"
    )));
    ControlFlow::Break(true)
}
