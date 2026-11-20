fn catfish_goals(mob: &mut Mob) {
    use super::{control::{LookKind, MoveKind}, goals::{Avoid, AvoidEntity, MeleeAttack, NearestTarget, Prey},
        nav::naturalist_catfish_state as catfish};
    if mob.kind.id != "naturalist:catfish" {
        return;
    }
    mob.ai.mov.kind = MoveKind::SmoothSwimming {
        max_turn_x: 1000.0, max_turn_y: 5.0, in_water: 0.02, outside: 0.1, gravity: false,
    };
    mob.ai.look.kind = LookKind::SmoothSwimming(5.0);
    let goals = &mut mob.ai.goals;
    goals.add(1, AvoidEntity::new(Avoid::Player, 6.0, 1.0, 1.5));
    goals.add(1, AvoidEntity::new(Avoid::Species(vec![Species::Axolotl]), 6.0, 1.0, 1.5));
    goals.add(2, MeleeAttack::new(1.4, false));
    let target = NearestTarget::new(Prey::Filter(|seen| {
        matches!(seen.species, Species::TropicalFish | Species::Tadpole)
            || seen.species == Species::Cod && seen.variant != catfish::SNAPSHOT
                && seen.variant != crate::mobs::nav::naturalist_piranha_state::SNAPSHOT
                && seen.variant != crate::mobs::nav::naturalist_blobfish_state::SNAPSHOT
                && seen.variant != crate::mobs::nav::naturalist_jellyfish_state::SNAPSHOT
                && seen.variant != catfish::STARFISH_SNAPSHOT
                && seen.variant != catfish::CLAM_SNAPSHOT
                && seen.variant != catfish::ANGLERFISH_SNAPSHOT
                && seen.variant != crate::mobs::nav::naturalist_piranha_state::RAY_SNAPSHOT
    }), true).named("NaturalistCatfishHunt").gate(|mob, _| catfish::cooldown(&mob.ai.nav) <= 0);
    mob.ai.targets.add(1, target);
}
