fn rhino_ai_tick(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    use horizon_client::mobs::goals;
    use horizon_core::nbt::Tag;
    use mod_api::Host;
    if matches!(kind, "naturalist:lion" | "naturalist:tiger") {
        let saved_cooldown = game.persist.raw.get(&entity)
            .and_then(|raw| raw.get("HuntingCooldown"))
            .and_then(horizon_world::datafix::dynamic::as_number)
            .map(|value| value.int_value().max(0));
        let next_cooldown = if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
            let ticks = saved_cooldown.or_else(|| {
                horizon_client::mobs::animal_brain::hunting_cooldown(mob)
                    .map(|ticks| ticks.clamp(0, i64::from(i32::MAX)) as i32)
            }).unwrap_or(0);
            if ticks > 0 {
                // Lions and Tigers use the Java HuntingCooldown field, but their
                // custom profiles don't enter Horizon's vanilla animal-brain tick.
                horizon_client::mobs::animal_brain::set_hunting_cooldown(mob, i64::from(ticks - 1));
            }
            Some(ticks.saturating_sub(1))
        } else { None };
        if let Some(ticks) = next_cooldown {
            let raw = game.persist.raw.entry(entity).or_insert_with(|| Tag::Compound(Default::default()));
            if let Tag::Compound(fields) = raw { fields.insert("HuntingCooldown".into(), Tag::Int(ticks)); }
        }
        crate::game::naturalist_rhino_goals::install_large_cat(game, entity, kind);
        return;
    }
    if kind != "naturalist:rhino" { return; }
    crate::game::naturalist_rhino_goals::tick(game, entity);
    let Some(index) = game.mobs.iter().position(|mob| mob.id == entity) else { return; };
    crate::game::naturalist_rhino_goals::break_charge_blocks(game, index);
    let position = game.mobs[index].position;
    let calf_nearby = game.mobs.iter().any(|mob| {
        mob.id != entity && mob.kind.id == "naturalist:rhino" && mob.is_baby()
            && mob.alive() && mob.distance_sqr(position) <= 256.0
    });
    let has_target = game.mobs[index].ai.targets.has("nearest_target");
    let has_attack = game.mobs[index].ai.goals.has("NaturalistRhinoCharge");
    if calf_nearby {
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
            if !has_target {
                mob.ai.targets.add(1, goals::NearestTarget::player(true));
            }
            if !has_attack { mob.ai.goals.add(2, crate::game::naturalist_rhino_goals::RhinoCharge::default()); }
        }
    } else if has_target || has_attack {
        if let Some(entity_ref) = game.entity(entity) {
            if let Some(handle) = game.mob(entity_ref) {
                if has_target { handle.remove_target_goal(game, "nearest_target"); }
                if has_attack { handle.remove_goal(game, "NaturalistRhinoCharge"); }
            }
        }
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
            mob.ai.target = None;
        }
    }
}
