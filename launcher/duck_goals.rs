fn duck_goals(mob: &mut Mob) {
    use super::goals::{Breed, Float, FollowParent, LookAtPlayer, Panic, RandomLookAround, Stroll, Tempt};
    use std::sync::Arc;

    if mob.kind.id != "naturalist:duck" {
        return;
    }
    let kind = Arc::make_mut(&mut mob.kind);
    let fish: Arc<[String]> = ["minecraft:cod", "minecraft:salmon", "minecraft:tropical_fish",
        "minecraft:pufferfish", "naturalist:anglerfish", "naturalist:bass", "naturalist:blobfish",
        "naturalist:catfish", "naturalist:piranha"].into_iter().map(str::to_owned).collect();
    kind.food = fish.clone();
    kind.tempt = fish.clone();
    kind.water_slowdown = 0.9;
    mob.extra.color = u8::MAX;
    let goals = &mut mob.ai.goals;
    goals.clear();
    super::tame::naturalist_duck_pet_goals::add_goals(mob);
    let goals = &mut mob.ai.goals;
    goals.add(0, Float);
    goals.add(1, Panic::new(1.25));
    goals.add(2, Breed::new(1.0));
    goals.add(3, Tempt::new(1.0, fish, false));
    goals.add(4, FollowParent::new(1.1));
    goals.add(5, Stroll::swimming(1.0, 10));
    goals.add(6, Stroll::new(1.0));
    goals.add(7, LookAtPlayer::new(6.0));
    goals.add(8, RandomLookAround::default());
}
