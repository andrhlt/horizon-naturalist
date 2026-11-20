fn crab_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::{Damage, DamageKind, Species, Who, nav::naturalist_crab as crab, spawn_selection};
    use std::ops::ControlFlow;

    let Some(query) = command.strip_prefix("naturalist_fixture_crab ") else {
        return ControlFlow::Continue(());
    };
    if query == "prey" {
        let target = game.mobs.iter().find(|mob| mob.kind.id == "naturalist:blobfish")
            .and_then(|mob| match mob.ai.target { Some(Who::Mob(id)) => Some(id), _ => None })
            .and_then(|id| game.mobs.iter().find(|mob| mob.id == id))
            .map(|mob| mob.kind.id.as_str());
        let output = serde_json::json!({"target": target});
        game.feedback(Text::literal(format!("Crab fixture {}", output)));
        return ControlFlow::Break(true);
    }
    let Some(index) = game.mobs.iter().position(|mob| mob.kind.id == crab::KIND && mob.alive()
        && mob.is_baby() == (query == "baby"))
    else {
        return ControlFlow::Break(false);
    };
    let mob = &game.mobs[index];
    let output = if query == "attack" {
        let attacker = mob.id;
        let at = mob.position.components();
        let zombie = game.mobs.iter().find(|mob| mob.kind.id == "minecraft:zombie" && mob.alive()).unwrap().id;
        let source = Damage { kind: DamageKind::MobAttack, attacker: Some(Who::Mob(attacker)),
            from: Some(at), attacker_species: Some(Species::Wolf) };
        let landed = game.mob_attacks_mob(zombie, 2.0, source);
        let mob = game.mobs.iter().find(|mob| mob.id == attacker).unwrap();
        serde_json::json!({"weapon_hit_landed": landed,
            "weapon_damage_on_hit": mob.extra.main_hand_stack().map_or(0, |item| item.damage)})
    } else {
        let (width, height, eye) = mob.dims();
        let dimensions = [width, height, eye].map(|size| (size * 10000.0).round() as u32);
        let mut sample = mob.clone();
        sample.velocity = horizon_core::geometry::Vec3::ZERO;
        sample.knockback(0.4, 1.0, 0.0, &Damage::of(DamageKind::Generic));
        serde_json::json!({
            "hides": crab::hides(mob), "dances": crab::state(&mob.ai.nav).dancing,
            "wearing": mob.extra.main_hand_stack().map(|stack| stack.item.as_ref()),
            "cod_food": mob.kind.food.iter().any(|food| food == "minecraft:cod"),
            "crab_food": mob.kind.food.iter().any(|food| food == "minecraft:tropical_fish"),
            "weapon_wanted": !crab::holding_weapon(mob) && crab::WEAPON_TAGS.iter().any(|tag|
                game.items().item_tags.get(*tag).is_some_and(|entry| entry.contains("minecraft:stone_sword"))),
            "stick_wanted": !crab::holding_weapon(mob) && crab::WEAPON_TAGS.iter().any(|tag|
                game.items().item_tags.get(*tag).is_some_and(|entry| entry.contains("minecraft:stick"))),
            "knock": (sample.velocity.x.abs() * 1000.0).round() as i32,
            "portal": horizon_client::mobs::mob_can_use_portal(mob),
            "cluster": spawn_selection::max_cluster_size(Species::Cow),
            "dims": dimensions,
        })
    };
    game.feedback(Text::literal(format!("Crab fixture {}", output)));
    ControlFlow::Break(true)
}
