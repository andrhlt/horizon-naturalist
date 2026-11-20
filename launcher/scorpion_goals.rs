fn scorpion_goals(mob: &mut Mob) {
    use super::{
        Seen, drops::is_monster, hostile::mob_brightness, naturalist_scorpion::*,
        goals::{Float, HurtByTarget, LookAtPlayer, MeleeAttack, NearestTarget, Prey, RandomLookAround, Stroll},
    };
    let desert = match mob.kind.id.as_str() {
        DESERT => true,
        JUNGLE => false,
        _ => return,
    };
    let kind = std::sync::Arc::make_mut(&mut mob.kind);
    kind.food = Default::default();
    kind.tempt = Default::default();
    let goals = &mut mob.ai.goals;
    goals.clear();
    goals.add(0, Float);
    goals.add(1, MeleeAttack::new(1.35, true));
    goals.add(2, Stroll::avoiding_water(1.0));
    goals.add(3, LookAtPlayer::new(6.0));
    goals.add(4, RandomLookAround::default());

    let targets = &mut mob.ai.targets;
    let same_species: fn(&Seen) -> bool = if desert {
        |seen| seen.effects.contains(&DESERT)
    } else {
        |seen| seen.effects.contains(&JUNGLE)
    };
    targets.clear();
    targets.add(1, HurtByTarget::new().alert().alert_if(same_species));
    targets.add(2, NearestTarget::new(Prey::Filter(|seen| seen.effects.contains(&PREY)), true).reach());
    targets.add(3, NearestTarget::player(true).reach().when(|mob, world|
        !from_hand(&mob.ai.nav) && mob_brightness(mob, world) < 0.5
    ));
    if desert {
        targets.add(4, NearestTarget::new(Prey::Filter(|seen| is_monster(seen.species)), true)
            .reach().when(|mob, _| from_hand(&mob.ai.nav)));
    }
}
