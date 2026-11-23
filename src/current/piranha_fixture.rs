fn piranha_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    let Some(query) = command.strip_prefix("naturalist_fixture_piranha ") else { return ControlFlow::Continue(()); };
    let fish: Vec<_> = game.mobs.iter().filter(|mob| mob.kind.id == "naturalist:piranha" && mob.alive()).collect();
    let school_sizes: Vec<_> = fish.iter().filter(|mob| mob.extra.followers > 0)
        .map(|mob| mob.extra.followers + 1).collect();
    let report = serde_json::json!({
        "query": query,
        "members": fish.len(),
        "leaders": school_sizes.len(),
        "largest_school": school_sizes.iter().copied().max().unwrap_or(1),
        "attached_followers": school_sizes.iter().map(|size| size - 1).sum::<i32>(),
        "targeted": fish.iter().filter(|mob| mob.ai.target.is_some()).count(),
    });
    game.feedback(Text::literal(format!("Piranha fixture {report}")));
    ControlFlow::Break(true)
}
