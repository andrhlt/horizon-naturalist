fn giant_isopod_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    use horizon_client::mobs::{Damage, DamageKind, nav::naturalist_giant_isopod as isopod};
    let Some(query) = command.strip_prefix("naturalist_fixture_giant_isopod ") else {
        return ControlFlow::Continue(());
    };
    let output = if query == "spawn" {
        game.naturalist_fixture_giant_isopod_spawn()
    } else {
        let needle = if query == "baby" { "baby" } else { "adult" };
        let mob = game.mobs.iter_mut().find(|mob|
            mob.kind.id == isopod::KIND && mob.alive() && mob.is_baby() == (needle == "baby")).unwrap();
        match query {
            "near" | "outside" | "baby" | "after_9" | "after_10" | "creative" | "invisible" | "wall" => {
                let key = match query {
                    "near" => "near_hides", "outside" => "outside_hides", "baby" => "baby_hides",
                    "after_9" => "holds_after_9", "after_10" => "holds_after_10",
                    "creative" => "creative_hides", "invisible" => "invisible_player_hides", _ => "wall_hides",
                };
                serde_json::json!({key: isopod::hiding(mob)})
            }
            "knock_near" | "knock_outside" => {
                let mut individual = mob.clone();
                individual.velocity = horizon_core::geometry::Vec3::ZERO;
                individual.knockback(0.4, 1.0, 0.0, &Damage::of(DamageKind::Generic));
                let key = if query == "knock_near" { "near_knockback" } else { "outside_knockback" };
                serde_json::json!({key: (individual.velocity.x.abs() * 1000.0).round() as i32})
            }
            "food" => serde_json::json!({
                "cod_food": mob.kind.food.iter().any(|item| item == "minecraft:cod"),
                "kelp_food": mob.kind.food.iter().any(|item| item == "minecraft:kelp"),
            }),
            _ => return ControlFlow::Continue(()),
        }.to_string()
    };
    game.feedback(Text::literal(format!("Isopod fixture {output}")));
    ControlFlow::Break(true)
}
