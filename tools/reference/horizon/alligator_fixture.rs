fn alligator_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::{Damage, DamageKind, Species, Who, spawn_selection};
    use horizon_core::{geometry::Vec3, position::BlockPos};
    use horizon_world::level::LevelAccess;
    use std::ops::ControlFlow;

    let Some(query) = command.strip_prefix("naturalist_fixture_alligator ") else {
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
        let mut babies = game.mobs.iter().filter(|m| m.kind.id == "naturalist:alligator"
            && m.position.x >= 11.0 && m.position.x <= 12.0
            && m.position.z >= 5.0 && m.position.z <= 6.0
            && m.is_baby()).map(|m| (m.extra.age, ((m.position.x - 11.0) * 100.0).round() as i32))
            .collect::<Vec<_>>();
        babies.sort_unstable();
        serde_json::json!({"stage": stage,
            "hatch_baby_ages": babies.iter().map(|&(age, _)| age).collect::<Vec<_>>(),
            "hatch_offsets": babies.iter().map(|&(_, x)| x).collect::<Vec<_>>()})
    } else {
        let Some(index) = game.mobs.iter().position(|m| m.kind.id == "naturalist:alligator" &&
            m.alive() && m.position.x < 10.0 && m.is_baby() == (query == "baby"))
        else {
            return ControlFlow::Break(false);
        };
        if query == "kill" {
            let attacker = game.mobs[index].id;
            let at = game.mobs[index].position.components();
            let prey = game.mobs.iter().find(|m| m.kind.id == "minecraft:frog" && m.alive()).unwrap().id;
            let source = Damage { kind: DamageKind::MobAttack, attacker: Some(Who::Mob(attacker)),
                from: Some(at), attacker_species: Some(Species::Cow) };
            let _ = game.mob_attacks_mob(prey, 50.0, source);
        }
        let mob = &game.mobs[index];
        let mut sample = mob.clone();
        sample.velocity = Vec3::ZERO;
        sample.knockback(0.4, 1.0, 0.0, &Damage::of(DamageKind::Generic));
        let (w, h, e) = mob.dims();
        let dims = [w, h, e].map(|v| (v * 10000.0).round() as i32);
        let target = match mob.ai.target {
            Some(Who::Mob(id)) => game.mobs.iter().find(|m| m.id == id).map(|m| m.kind.id.as_str()),
            Some(Who::Player) => Some("player"),
            _ => None,
        };
        let food = ["beef", "porkchop", "chicken", "rabbit", "cod", "mutton", "cooked_beef",
            "rotten_flesh"].map(|name| mob.kind.food.iter().any(|food| food == &format!("minecraft:{name}")));
        serde_json::json!({
            "dims": dims,
            "head_yaw": mob.kind.max_head_y_rot, "cluster": spawn_selection::max_cluster_size(Species::Cow),
            "food": food, "knockback": (sample.velocity.x.abs() * 1000.0).round() as i32,
            "hunting": horizon_client::mobs::nav::naturalist_catfish_state::cooldown(&mob.ai.nav),
            "laying": mob.extra.flag2, "counter": mob.extra.counter,
            "target": target, "has_egg": mob.extra.flag, "in_love": mob.extra.in_love,
        })
    };
    game.feedback(Text::literal(format!("Alligator fixture {}", output)));
    ControlFlow::Break(true)
}
