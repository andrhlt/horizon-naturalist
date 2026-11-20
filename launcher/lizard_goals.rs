fn lizard_goals(mob: &mut Mob) {
    use super::goals::{Float, LookAtPlayer, RandomLookAround, Stroll};
    use std::sync::Arc;
    if !matches!(mob.kind.id.as_str(), "naturalist:lizard" | "naturalist:lizard_tail") {
        return;
    }
    let tail = mob.kind.id == "naturalist:lizard_tail";
    let kind = Arc::make_mut(&mut mob.kind);
    kind.food = Default::default();
    kind.tempt = Default::default();
    mob.ai.goals.clear();
    mob.ai.targets.clear();
    if tail {
        kind.flags.animal = false;
        return;
    }
    let tempt = Arc::from(["minecraft:spider_eye".to_owned()]);
    kind.tempt = Arc::clone(&tempt);
    mob.extra.color = u8::MAX;
    super::tame::naturalist_duck_pet_goals::add_lizard_goals(mob);
    let goals = &mut mob.ai.goals;
    goals.add(1, Float);
    goals.add(3, super::naturalist_lizard::LizardTempt::new(tempt));
    goals.add(8, Stroll::avoiding_water(1.0));
    goals.add(9, LookAtPlayer::new(8.0));
    goals.add(10, RandomLookAround::default());
}
