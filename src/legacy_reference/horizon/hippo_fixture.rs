fn hippo_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::{Damage, DamageKind, Species, spawn_selection};
    use horizon_core::geometry::Vec3;

    let Some(query) = command.strip_prefix("naturalist_fixture_hippo ") else {
        return std::ops::ControlFlow::Continue(());
    };

    if matches!(query, "spawn-natural" | "spawn-generation") {
        let allowed = game.naturalist_fixture_hippo_spawn(query == "spawn-generation");
        game.feedback(Text::literal(format!("Hippo fixture {}", serde_json::json!({"allowed": allowed}))));
        return std::ops::ControlFlow::Break(true);
    }
    let baby = query == "baby";
    let records = game.mobs.iter().filter(|mob| mob.kind.id == "naturalist:hippo" && mob.alive()
        && (query == "all" || mob.is_baby() == baby)).map(|mob| {
        let mut sample = mob.clone();
        sample.velocity = Vec3::ZERO;
        sample.knockback(0.4, 1.0, 0.0, &Damage::of(DamageKind::Generic));
        let (w, h, e) = mob.dims();
        let dims = [w, h, e].map(|v| (v * 10000.0).round() as i32);
        let food = ["minecraft:melon", "minecraft:melon_slice", "minecraft:glistering_melon_slice",
            "minecraft:carrot"].map(|id| mob.kind.food.iter().any(|food| food == id));
        serde_json::json!({
            "id": mob.id, "position": mob.position.components(), "dims": dims, "food": food,
            "occupied": mob.extra.main_hand_stack().map(|stack| [stack.item.to_string(), stack.count.to_string()]),
            "age": mob.extra.age, "wet": mob.fluids.in_water, "tame": mob.extra.tame,
            "owner": mob.extra.owner, "sitting": mob.extra.sitting,
            "following": horizon_client::mobs::nav::naturalist_duck_pet::is_following(&mob.ai.nav),
            "threshold": (mob.fluid_jump_threshold() * 100.0).round() as i32,
            "knockback": (sample.velocity.x.abs() * 1000.0).round() as i32,
            "portal": horizon_client::mobs::mob_can_use_portal(mob),
            "cluster": spawn_selection::max_cluster_size(Species::Cow),
            "eating": horizon_client::mobs::nav::naturalist_hippo_state::state(&mob.ai.nav).eating,
        })
    }).collect::<Vec<_>>();
    game.feedback(Text::literal(format!("Hippo fixture {}", serde_json::json!({"mobs": records}))));
    std::ops::ControlFlow::Break(true)
}
