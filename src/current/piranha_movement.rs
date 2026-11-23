fn piranha_movement(game: &mut Game<'_>, entity: u32, kind: &str) {
    use horizon_client::mobs::control::{LookKind, MoveKind};
    use mod_api::Host;
    if kind != "naturalist:piranha" { return; }
    if let Some(mob) = game.entity(entity).and_then(|entity| game.mob(entity)) {
        mob.remove_goal(game, "FollowFlockLeaderGoal");
    }
    let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) else { return; };
    if !mob.ai.goals.has("NaturalistPiranhaSchool") {
        let goal = crate::game::naturalist_piranha_school::PiranhaSchool::new(mob);
        mob.ai.goals.add(5, goal);
    }
    if !matches!(mob.ai.mov.kind, MoveKind::SmoothSwimming { .. }) {
        mob.ai.mov.kind = MoveKind::SmoothSwimming { max_turn_x: 1000.0, max_turn_y: 5.0, in_water: 0.02, outside: 0.1, gravity: false };
    }
    mob.ai.look.kind = LookKind::SmoothSwimming(5.0);
}
