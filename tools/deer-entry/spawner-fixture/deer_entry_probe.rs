fn deer_entry_probe(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::finalize::{Attr, modifier_amount};
    use horizon_core::random::RandomSource;
    use serde_json::json;
    use std::ops::ControlFlow;

    let Some(input) = command.strip_prefix("naturalist_fixture_deer_entry ") else {
        return ControlFlow::Continue(());
    };
    if let Some(seed) = input.strip_prefix("seed ").and_then(|seed| seed.parse::<i64>().ok()) {
        game.world.random.set_seed(seed);
        game.feedback(Text::literal("Deer entry seeded"));
        return ControlFlow::Break(true);
    }
    if input == "egg" {
        let spawned = game.spawn_mob_spawned(
            "naturalist:deer", [8.5, 100.0, 8.5], super::creatures::SpawnYaw::Created,
        );
        return ControlFlow::Break(spawned.is_some());
    }
    if input == "spawner" {
        let result = game.tick_spawner((8, 100, 8));
        return ControlFlow::Break(result.is_ok());
    }
    if input != "observe" {
        return ControlFlow::Continue(());
    }
    let Some(deer) = game.mobs.iter_mut().rfind(|mob| mob.kind.id == "naturalist:deer" && !mob.removed && mob.alive()) else {
        return ControlFlow::Break(false);
    };
    let variant = game.persist.raw.get(&deer.id).and_then(|tag|
        horizon_client_core::nbt_value::text(tag.get("Variant"))).unwrap_or_else(|| deer.kind.id.clone());
    let follow_bonus = modifier_amount(deer, Attr::FollowRange, "minecraft:random_spawn_bonus");
    let data = json!({
        "pos": deer.position.components(),
        "variant": variant,
        "follow_bonus": follow_bonus,
        "left_handed": deer.extra.left_handed,
        "age": deer.extra.age,
        "yaw": deer.yaw,
        "next": game.world.random.next_long(),
    });
    deer.removed = true;
    game.feedback(Text::literal(format!("Naturalist Deer entry {data}")));
    ControlFlow::Break(true)
}
