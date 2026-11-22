fn firefly_glow_status(game: &Game<'_>, line: &str) -> std::ops::ControlFlow<Result<Option<Value>>> {
    use horizon_client::mobs::goals::naturalist_firefly_behavior::glow_frame;
    if line != "status" || !game.options.headless {
        return std::ops::ControlFlow::Continue(());
    }
    let mut status = game.control_status();
    let fireflies: Vec<_> = game
        .mobs
        .iter()
        .filter(|mob| mob.kind.id == "naturalist:firefly" && !mob.removed)
        .map(|mob| {
            let (frame, emissive) = glow_frame(mob);
            serde_json::json!([mob.id, frame, emissive])
        })
        .collect();
    status["naturalist_firefly_glow"] = fireflies.into();
    std::ops::ControlFlow::Break(Ok(Some(status)))
}
