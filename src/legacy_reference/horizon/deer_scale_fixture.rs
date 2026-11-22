fn deer_scale_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if command != "naturalist_fixture_deer_scale" {
        return ControlFlow::Continue(());
    }
    let answer: Vec<_> = game.mobs.iter().filter(|mob| mob.kind.id == "naturalist:deer").map(|mob| {
        let (width, height, eye) = mob.dims();
        (mob.is_baby(), [f64::from(mob.model_scale()), width, height, eye])
    }).collect();
    game.feedback(Text::literal(format!("naturalist_fixture_deer_scale {}", serde_json::json!(answer))));
    ControlFlow::Break(true)
}
