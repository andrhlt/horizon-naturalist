fn ray_goals(mob: &mut Mob) {
    use super::{control::{LookKind, MoveKind}, goals::{HurtByTarget, MeleeAttack, Stroll}};
    if mob.kind.id != "naturalist:ray" {
        return;
    }
    mob.ai.mov.kind = MoveKind::SmoothSwimming {
        max_turn_x: 1000.0, max_turn_y: 5.0, in_water: 0.02, outside: 0.1, gravity: false,
    };
    mob.ai.look.kind = LookKind::SmoothSwimming(5.0);
    mob.ai.goals.clear();
    mob.ai.goals.add(1, MeleeAttack::new(1.2, true));
    mob.ai.goals.add(2, Stroll::swimming(1.0, 20));
    mob.ai.targets.add(1, HurtByTarget::new());
}
