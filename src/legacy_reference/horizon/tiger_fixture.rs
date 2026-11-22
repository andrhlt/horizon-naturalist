fn tiger_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::{Who, goals::naturalist_cat_goals as tiger, nav::naturalist_catfish_state as hunt};
    use std::ops::ControlFlow;

    let Some(query) = command.strip_prefix("naturalist_fixture_tiger ") else { return ControlFlow::Continue(()) };
    let Some(mob) = game.mobs.iter().find(|mob| mob.alive()
        && mob.kind.id == tiger::KIND && mob.is_baby() == (query == "baby"))
    else { return ControlFlow::Break(false) };
    let world = game.mob_world_base(&[]);
    let (w, h, e) = mob.dims();
    let dims = [w, h, e].map(|n| (n * 10000.0).round() as i32);
    let food = [
        "minecraft:beef", "minecraft:cooked_mutton", "minecraft:chicken", "minecraft:cooked_chicken",
        "minecraft:rabbit", "minecraft:cod", "minecraft:cooked_salmon", "minecraft:rotten_flesh",
        "minecraft:wheat", "minecraft:golden_carrot", "naturalist:bushmeat", "naturalist:cooked_bushmeat",
    ].map(|id| mob.kind.food.iter().any(|value| value == id));
    let target = match mob.ai.target {
        Some(Who::Mob(id)) => game.mobs.iter().find(|m| m.id == id).map(|m| m.kind.id.as_str()),
        Some(Who::Player) => Some("player"),
        _ => None,
    };
    let result = serde_json::json!({
        "dims": dims, "food": food, "food_ids": mob.kind.food.as_ref(),
        "ambient_interval": mob.ambient_interval(), "clock": world.day_time,
        "can_sleep": tiger::can_sleep(mob, &world), "sleeping": tiger::sleeping(mob),
        "stalking": tiger::stalking(mob), "target": target, "cooldown": hunt::cooldown(&mob.ai.nav),
        "pos": mob.position.components(), "health": mob.health,
    });
    game.feedback(Text::literal(format!("Tiger fixture {result}")));
    ControlFlow::Break(true)
}
