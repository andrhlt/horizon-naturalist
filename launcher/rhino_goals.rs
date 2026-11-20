fn rhino_goals(mob: &mut Mob) {
    use super::{
        goals::{Float, FollowParent, HurtByTarget, LookAtPlayer, MeleeAttack, NearestTarget,
            Panic, Prey, RandomLookAround, Stroll},
        naturalist_rhino::{Charge, KIND, PUMPKIN, Prepare, protects_baby},
    };
    if mob.kind.id != KIND {
        return;
    }
    let kind = std::sync::Arc::make_mut(&mut mob.kind);
    kind.food = Default::default();
    kind.tempt = Default::default();
    kind.step_height = 1.0;
    mob.ai.goals.clear();
    let goals = &mut mob.ai.goals;
    goals.add(0, Float);
    goals.add(1, MeleeAttack::new(1.2, false));
    goals.add(2, Prepare);
    goals.add(3, Charge::default());
    goals.add(3, Panic::new(1.25));
    goals.add(4, FollowParent::new(1.1));
    goals.add(5, Stroll::avoiding_water(1.0));
    goals.add(6, LookAtPlayer::new(6.0));
    goals.add(7, RandomLookAround::default());
    let targets = &mut mob.ai.targets;
    targets.clear();
    targets.add(1, HurtByTarget::new().polar_bear().alert_if(|seen| seen.effects.contains(&KIND)));
    targets.add(2, NearestTarget::player(true).reach().when(protects_baby));
    targets.add(3, NearestTarget::player(false).interval(0).when(|_, world|
        world.player.as_ref().is_some_and(|player| player.head.as_deref() == Some("minecraft:carved_pumpkin"))));
    targets.add(3, NearestTarget::new(Prey::Filter(|seen| seen.effects.contains(&PUMPKIN)), false).interval(0));
}
