fn bass_goals(mob: &mut Mob) {
    use super::{
        Species,
        control::{LookKind, MoveKind},
        goals::{Avoid, AvoidEntity, MeleeAttack, NearestTarget, Prey},
        nav::naturalist_bass_state as bass,
    };

    if mob.kind.id != "naturalist:bass" {
        return;
    }
    mob.ai.mov.kind = MoveKind::SmoothSwimming {
        max_turn_x: 1000.0, max_turn_y: 5.0, in_water: 0.02, outside: 0.1, gravity: false,
    };
    mob.ai.look.kind = LookKind::SmoothSwimming(5.0);
    let goals = &mut mob.ai.goals;
    goals.add(1, AvoidEntity::new(Avoid::Player, 6.0, 1.0, 1.5));
    goals.add(1, AvoidEntity::new(Avoid::Species(vec![Species::Axolotl]), 6.0, 1.0, 1.5));
    let mut larger_bass = AvoidEntity::new(Avoid::Species(vec![Species::Cod]), 10.0, 1.3, 1.7);
    *larger_bass.__horizon_mixins_naturalist_bass_avoid_larger_mut() = true;
    goals.add(1, larger_bass);
    goals.add(3, MeleeAttack::new(1.4, true));
    let target = NearestTarget::new(Prey::Filter(|_| false), true)
        .named("NaturalistBassHunt")
        .gate(|mob, _| mob.extra.variant > 0 && bass::cooldown(&mob.ai.nav) <= 0);
    mob.ai.targets.add(1, target);
}
