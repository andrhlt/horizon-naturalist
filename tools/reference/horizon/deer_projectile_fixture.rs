fn deer_projectile_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if command != "naturalist_fixture_deer_projectiles" {
        return ControlFlow::Continue(());
    }
    let mut herd: Vec<_> = game.mobs.iter()
        .filter(|mob| mob.kind.id == "naturalist:deer" && !mob.removed && mob.alive())
        .collect();
    herd.sort_by(|a, b| a.position.x.total_cmp(&b.position.x));
    let rows: Vec<_> = herd.iter().map(|mob| (mob.extra.ints[0], mob.health)).collect();
    game.feedback(Text::literal(format!("Naturalist projectile herd {}", serde_json::json!(rows))));
    ControlFlow::Break(true)
}
