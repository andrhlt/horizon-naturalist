fn piranha_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    let Some(query) = command.strip_prefix("naturalist_fixture_piranha ") else { return ControlFlow::Continue(()); };
    if let Some(tag) = query.strip_prefix("start ") {
        let id = game.mobs.iter().find(|mob| {
            mob.kind.id == "naturalist:piranha"
                && game.tags_of(Target::Mob(mob.id)).iter().any(|value| value == tag)
        }).map(|mob| mob.id);
        if let Some(id) = id {
            if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) {
                mob.ai.goals.set_start_delay("NaturalistPiranhaSchool", 0);
            }
        }
        game.feedback(Text::literal(format!("Piranha fixture start {id:?}")));
        return ControlFlow::Break(true);
    }
    let fish: Vec<_> = game.mobs.iter().filter(|mob| mob.kind.id == "naturalist:piranha" && mob.alive()).collect();
    let attacker_id = game.mobs.iter().find(|mob| game.tags_of(Target::Mob(mob.id)).iter().any(|tag| tag == "app2129_attacker")).map(|mob| mob.id);
    let retaliation_target = game.mobs.iter().find(|mob| game.tags_of(Target::Mob(mob.id)).iter().any(|tag| tag == "app2129_retaliator"))
        .map(|mob| (mob.id, format!("{:?}", mob.ai.target)));
    let school_sizes: Vec<_> = fish.iter().filter(|mob| mob.extra.followers > 0)
        .map(|mob| mob.extra.followers + 1).collect();
    let report = serde_json::json!({
        "query": query,
        "members": fish.len(),
        "leaders": school_sizes.len(),
        "largest_school": school_sizes.iter().copied().max().unwrap_or(1),
        "attached_followers": school_sizes.iter().map(|size| size - 1).sum::<i32>(),
        "targeted": fish.iter().filter(|mob| mob.ai.target.is_some()).count(),
        "attacker_id": attacker_id,
        "retaliator_id": retaliation_target.as_ref().map(|value| value.0),
        "retaliator_target": retaliation_target.map(|value| value.1),
    });
    game.feedback(Text::literal(format!("Piranha fixture {report}")));
    ControlFlow::Break(true)
}
