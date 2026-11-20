fn crab_goals(mob: &mut Mob) {
    use super::{goals::{Breed, Float, HurtByTarget, LookAtPlayer, MeleeAttack, Panic, RandomLookAround, Stroll, Tempt},
        nav::naturalist_crab as crab};
    use std::sync::Arc;

    if mob.kind.id != crab::KIND {
        return;
    }
    let food: Arc<[String]> = Arc::from([crab::FOOD.to_owned()]);
    let kind = Arc::make_mut(&mut mob.kind);
    kind.food = food.clone();
    kind.tempt = food.clone();
    mob.extra.pick_up_loot = true;
    let goals = &mut mob.ai.goals;
    goals.clear();
    goals.add(0, Float);
    goals.add(2, MeleeAttack::new(1.65, true));
    goals.add(3, Panic::new(1.5));
    goals.add(5, Breed::new(1.0));
    goals.add(6, Tempt::new(1.2, food, false));
    goals.add(8, Stroll::avoiding_water(1.0));
    goals.add(9, LookAtPlayer::new(6.0));
    goals.add(10, RandomLookAround::default());
    mob.ai.targets.clear();
    mob.ai.targets.add(3, HurtByTarget::new());
    super::tame::naturalist_duck_pet_goals::add_crab_goals(mob);
}
