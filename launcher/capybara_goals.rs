fn capybara_goals(mob: &mut Mob) {
    use super::goals::{Breed, Float, FollowParent, LookAtPlayer, Panic, RandomLookAround, Stroll, Tempt};
    use std::sync::Arc;
    if mob.kind.id != "naturalist:capybara" {
        return;
    }
    let food: Arc<[String]> = Arc::from(["minecraft:melon_slice".to_owned()]);
    let kind = Arc::make_mut(&mut mob.kind);
    kind.food = food.clone();
    kind.tempt = food.clone();
    kind.water_slowdown = 0.9;
    kind.fall_damage_multiplier = 0.0;
    mob.extra.color = u8::MAX;
    mob.ai.goals.clear();
    super::tame::naturalist_duck_pet_goals::add_capybara_goals(mob);
    let goals = &mut mob.ai.goals;
    goals.add(0, Float);
    goals.add(1, Panic::new(1.4));
    goals.add(2, Breed::new(1.0));
    goals.add(3, Tempt::new(1.2, food, false));
    goals.add(4, FollowParent::new(1.1));
    goals.add(5, super::naturalist_capybara_sleep::Sleep);
    goals.add(6, Stroll::swimming(1.0, 10));
    goals.add(6, Stroll::new(1.0));
    goals.add(7, LookAtPlayer::new(6.0));
    goals.add(8, RandomLookAround::default());
}
