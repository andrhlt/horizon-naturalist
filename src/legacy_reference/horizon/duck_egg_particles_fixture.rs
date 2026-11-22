fn duck_egg_particles_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if command != "naturalist_fixture_duck_egg_particles" {
        return ControlFlow::Continue(());
    }
    let at = [8.5, 101.5, 8.5];
    game.particles.requested = Some(Vec::new());
    game.item_burst(at, "naturalist:duck_egg");
    let requested = game.particles.requested.take().unwrap_or_default();
    let velocities = requested.iter().map(|(_, _, velocity)| *velocity).collect::<Vec<_>>();
    game.feedback(Text::literal(format!(
        "Duck egg particles {}", serde_json::json!(velocities)
    )));
    ControlFlow::Break(true)
}
