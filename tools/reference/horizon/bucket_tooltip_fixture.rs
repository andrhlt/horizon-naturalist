fn bucket_tooltip_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if command != "naturalist_fixture_bucket_tooltip" {
        return ControlFlow::Continue(());
    }
    let mut lines = Vec::new();
    if let Some(stack) = game.held() {
        game.native_tooltip(stack, &mut lines);
    }
    let rendered = lines.iter().map(|(text, color)| {
        let key = text.translation_key().map_or_else(|| text.plain(), str::to_owned);
        format!("{key} {}", color & 0xFFFFFF)
    }).collect::<Vec<_>>();
    game.feedback(Text::literal(format!("Bucket tooltip fixture {}", serde_json::to_string(&rendered).unwrap())));
    ControlFlow::Break(true)
}
