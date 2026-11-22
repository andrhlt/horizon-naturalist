fn blobfish_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if command != "naturalist_fixture_blobfish_spawn" {
        return ControlFlow::Continue(());
    }
    let output = game.naturalist_fixture_blobfish_spawn();
    game.feedback(Text::literal(format!("Blobfish fixture {output}")));
    ControlFlow::Break(true)
}
