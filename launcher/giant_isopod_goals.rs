fn giant_isopod_goals(mob: &mut Mob) {
    use super::{goals::{Avoid, AvoidEntity, Breed, FollowParent, LookAtPlayer, Panic, RandomLookAround, Stroll, Tempt},
        nav::naturalist_giant_isopod as isopod, path::PathType};
    use std::sync::Arc;

    if mob.kind.id != isopod::KIND {
        return;
    }
    let food: Arc<[String]> = Arc::from([
        "minecraft:cod", "minecraft:cooked_cod", "minecraft:salmon", "minecraft:cooked_salmon",
        "minecraft:tropical_fish", "naturalist:catfish", "naturalist:cooked_catfish", "naturalist:bass",
        "naturalist:cooked_bass",
    ].map(str::to_owned));
    let kind = Arc::make_mut(&mut mob.kind);
    kind.food = food.clone();
    kind.tempt = food.clone();
    kind.flags.breathes_water = true;
    kind.water_slowdown = 0.92;
    mob.ai.nav.can_float = false;
    mob.set_malus(PathType::Water, 0.0);
    mob.set_malus(PathType::WaterBorder, 0.0);
    let goals = &mut mob.ai.goals;
    goals.clear();
    goals.add(0, Panic::new(1.25));
    goals.add(1, AvoidEntity::new(Avoid::Player, 5.0, 1.25, 1.5));
    goals.add(2, Breed::new(1.0));
    goals.add(3, Tempt::new(1.1, food, false));
    goals.add(4, FollowParent::new(1.1));
    goals.add(5, Stroll::new(1.0).interval(30));
    goals.add(6, LookAtPlayer::new(6.0));
    goals.add(7, RandomLookAround::default());
}
