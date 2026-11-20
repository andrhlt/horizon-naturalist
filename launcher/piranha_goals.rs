fn piranha_goals(mob: &mut Mob) {
    use super::{aquatic::FollowFlockLeader, control::{LookKind, MoveKind},
        goals::{HurtByTarget, MeleeAttack, NearestTarget, Prey, Stroll}};

    if mob.kind.id != "naturalist:piranha" {
        return;
    }
    mob.ai.mov.kind = MoveKind::SmoothSwimming {
        max_turn_x: 1000.0, max_turn_y: 5.0, in_water: 0.02, outside: 0.1, gravity: false,
    };
    mob.ai.look.kind = LookKind::SmoothSwimming(5.0);
    mob.ai.goals.clear();
    mob.ai.goals.add(1, MeleeAttack::new(1.4, true));
    let delay = FollowFlockLeader::next_start_tick(mob);
    mob.ai.goals.add(2, FollowFlockLeader::new(delay));
    mob.ai.goals.add(3, Stroll::swimming(1.0, 10));
    let school_leader = |mob: &Mob, world: &MobWorld<'_>| {
        mob.extra.leader.and_then(|id| world.seen(id)).is_none_or(|seen| !seen.alive)
    };
    mob.ai.targets.add(1, HurtByTarget::new());
    mob.ai.targets.add(2, NearestTarget::player(true).gate(school_leader).when(|_, world| {
        world.player.as_ref().is_some_and(|player| world.fluids(player.bounds, player.eye[1]).in_water)
    }));
    let prey = Prey::Filter(|seen| {
        seen.species == Species::Salmon || seen.species == Species::Cod
            && seen.variant < super::nav::naturalist_bass_state::SMALLEST_SNAPSHOT
    });
    mob.ai.targets.add(3, NearestTarget::new(prey, true).gate(school_leader));
}
