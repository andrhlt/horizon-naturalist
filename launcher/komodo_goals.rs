fn komodo_goals(mob: &mut Mob) {
    use super::{
        naturalist_komodo::{Bask, KIND, PREY, player_targeting},
        goals::{Breed, Float, FollowParent, HurtByTarget, LookAtPlayer, MeleeAttack, NearestTarget,
            Panic, Prey, RandomLookAround, Stroll, Tempt},
    };
    use std::sync::Arc;

    if mob.kind.id != KIND {
        return;
    }
    let foods = [
        "minecraft:beef", "minecraft:chicken", "minecraft:cooked_beef", "minecraft:cooked_chicken",
        "minecraft:cooked_mutton", "minecraft:cooked_porkchop", "minecraft:cooked_rabbit", "minecraft:mutton",
        "minecraft:porkchop", "minecraft:rabbit", "naturalist:bushmeat", "naturalist:clam_meat",
        "naturalist:cooked_bushmeat", "naturalist:cooked_clam_meat", "naturalist:cooked_crab_meat",
        "naturalist:cooked_drumstick", "naturalist:cooked_duck", "naturalist:cooked_lizard_tail",
        "naturalist:cooked_mammoth_meat", "naturalist:cooked_venison", "naturalist:crab_meat",
        "naturalist:drumstick", "naturalist:duck", "naturalist:lizard_tail", "naturalist:mammoth_meat",
        "naturalist:morsel", "naturalist:venison",
    ];
    let food: Arc<[String]> = foods.into_iter().map(str::to_owned).collect();
    let kind = Arc::make_mut(&mut mob.kind);
    kind.food = food.clone();
    kind.tempt = food.clone();
    kind.step_height = 1.0;
    let goals = &mut mob.ai.goals;
    goals.clear();
    goals.add(0, Float);
    goals.add(1, MeleeAttack::new(1.5, true));
    goals.add(2, Panic::new(2.0));
    goals.add(3, Breed::new(1.0));
    goals.add(4, Tempt::new(1.25, food, false));
    goals.add(5, Bask::default());
    goals.add(6, FollowParent::new(1.25));
    goals.add(7, Stroll::avoiding_water(1.0));
    goals.add(8, LookAtPlayer::new(6.0));
    goals.add(9, RandomLookAround::default());

    let targets = &mut mob.ai.targets;
    targets.clear();
    // The stock polar-bear goal alerts other adults only when a baby is hurt.
    targets.add(1, HurtByTarget::new().polar_bear().alert_if(|seen| seen.effects.contains(&KIND)));
    targets.add(2, NearestTarget::new(Prey::Filter(|seen| seen.effects.contains(&PREY)), true)
        .reach().when(|mob, _| !mob.is_baby()));
    targets.add(3, NearestTarget::player(true).reach().when(|mob, _| !mob.is_baby() && player_targeting(mob)));
}
