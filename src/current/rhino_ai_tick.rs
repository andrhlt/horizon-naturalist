fn rhino_ai_tick(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    use horizon_client::mobs::goals;
    use mod_api::Host;
    if kind != "naturalist:rhino" { return; }
    crate::game::naturalist_rhino_goals::tick(game, entity);
    let Some(index) = game.mobs.iter().position(|mob| mob.id == entity) else { return; };
    let position = game.mobs[index].position;
    let calf_nearby = game.mobs.iter().any(|mob| {
        mob.id != entity && mob.kind.id == "naturalist:rhino" && mob.is_baby()
            && mob.alive() && mob.distance_sqr(position) <= 256.0
    });
    let has_target = game.mobs[index].ai.targets.has("nearest_target");
    let has_attack = game.mobs[index].ai.goals.has("melee_attack");
    if calf_nearby {
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
            if !has_target {
                mob.ai.targets.add(1, goals::NearestTarget::player(true));
            }
            if !has_attack {
                mob.ai.goals.add(2, goals::MeleeAttack::new(1.0, true));
            }
        }
    } else if has_target || has_attack {
        if let Some(entity_ref) = game.entity(entity) {
            if let Some(handle) = game.mob(entity_ref) {
                if has_target { handle.remove_target_goal(game, "nearest_target"); }
                if has_attack { handle.remove_goal(game, "melee_attack"); }
            }
        }
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
            mob.ai.target = None;
        }
    }
}
