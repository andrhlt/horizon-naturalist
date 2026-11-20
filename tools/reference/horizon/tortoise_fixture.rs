fn tortoise_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::{Damage, DamageKind};
    use horizon_core::{geometry::Vec3, position::BlockPos};
    use horizon_world::level::LevelAccess;
    use std::ops::ControlFlow;

    let Some(query) = command.strip_prefix("naturalist_fixture_tortoise ") else {
        return ControlFlow::Continue(());
    };
    let output = if query == "hatch" {
        let pos = BlockPos::new(11, 92, 5);
        game.with_level(|_, level, ticks| {
            let mut state = level.block_state(pos);
            let mut handled = false;
            level.before_random_tick(ticks, pos, &mut state, &mut handled);
        });
        let _ = game.process_level_events();
        let state = game.world.block(pos.x, pos.y, pos.z);
        let row = game.pack.assets.registry.state(state).unwrap();
        let stage = row.properties.get("hatch").and_then(|v| v.parse::<i32>().ok()).unwrap_or(-1);
        let mut babies = game.mobs.iter().filter(|m| m.kind.id == "naturalist:tortoise"
            && (11.0..=12.0).contains(&m.position.x) && (5.0..=6.0).contains(&m.position.z)
            && m.is_baby()).map(|m| {
                let variant = horizon_client_core::nbt_value::text(
                    game.persist.raw.get(&m.id).and_then(|tag| tag.get("Variant")),
                );
                (((m.position.x - 11.0) * 100.0).round() as i32, m.extra.age, m.extra.tame, variant)
            }).collect::<Vec<_>>();
        babies.sort_by_key(|baby| baby.0);
        serde_json::json!({
            "stage": stage,
            "hatch_offsets": babies.iter().map(|baby| baby.0).collect::<Vec<_>>(),
            "hatch_baby_ages": babies.iter().map(|baby| baby.1).collect::<Vec<_>>(),
            "hatch_baby_tame": babies.iter().map(|baby| baby.2).collect::<Vec<_>>(),
            "hatch_baby_variants": babies.iter().map(|baby| &baby.3).collect::<Vec<_>>(),
        })
    } else {
        let Some(mob) = game.mobs.iter().find(|m| m.alive()
            && m.kind.id == "naturalist:tortoise" && m.position.x < 10.0 && m.is_baby() == (query == "baby"))
        else { return ControlFlow::Break(false) };
        use horizon_client::mobs::goals::naturalist_tortoise_goals as tortoise;
        let mut sampled = mob.clone();
        sampled.velocity = Vec3::ZERO;
        sampled.knockback(0.4, 1.0, 0.0, &Damage::of(DamageKind::Generic));
        let (w, h, e) = mob.dims();
        let food = ["bamboo", "dandelion", "brown_mushroom", "cactus", "wheat", "red_mushroom",
            "melon_slice", "seagrass"].map(|id| mob.kind.food.iter().any(|food| food == &format!("minecraft:{id}")));
        let dims = [w, h, e].map(|n| (n * 10000.0).round() as i32);
        serde_json::json!({
            "dims": dims,
            "head_yaw": mob.kind.max_head_y_rot, "food": food,
            "knockback": (sampled.velocity.x.abs() * 1000.0).round() as i32,
            "hidden": tortoise::hidden(mob), "has_egg": mob.extra.flag,
            "laying": mob.extra.flag2, "counter": mob.extra.counter, "in_love": mob.extra.in_love,
            "owner_heal": mob.health,
        })
    };
    game.feedback(Text::literal(format!("Tortoise fixture {output}")));
    ControlFlow::Break(true)
}
