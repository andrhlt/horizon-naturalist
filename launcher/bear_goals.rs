fn bear_goals(mob: &mut Mob) {
    use super::{
        goals::{Breed, FollowParent, HurtByTarget, LookAtPlayer, MeleeAttack, NearestTarget,
            Panic, Prey, RandomLookAround, Stroll},
        hostile::mob_brightness,
        naturalist_bears::{self as bear, BEAR, BLACK_BEAR},
        Seen,
    };
    use std::sync::Arc;

    let same_species: fn(&Seen) -> bool = match mob.kind.id.as_str() {
        BEAR => |seen| seen.effects.contains(&BEAR),
        BLACK_BEAR => |seen| seen.effects.contains(&BLACK_BEAR),
        _ => return,
    };
    let food: Arc<[String]> = ["minecraft:salmon", "minecraft:cooked_salmon", "minecraft:honeycomb",
        "minecraft:sweet_berries", "naturalist:venison", "alexsmobs:raw_moose_ribs", "alexsmobs:cooked_moose_ribs"]
        .into_iter().map(str::to_owned).collect();
    let kind = Arc::make_mut(&mut mob.kind);
    kind.food = food.clone();
    kind.tempt = food.clone();
    kind.water_slowdown = 0.98;
    kind.step_height = 1.0;
    mob.extra.color = u8::MAX;
    mob.extra.pick_up_loot = true;
    mob.extra.anger_end = -1;
    mob.ai.goals.clear();
    mob.ai.targets.clear();
    super::tame::naturalist_duck_pet_goals::add_bear_goals(mob);
    let goals = &mut mob.ai.goals;
    goals.add(0, bear::BearFloat);
    goals.add(1, Breed::new(1.0));
    goals.add(2, MeleeAttack::new(1.25, true));
    goals.add(3, bear::Sleep);
    goals.add(4, bear::BearTempt::new(food));
    goals.add(4, Panic::new(1.25));
    goals.add(5, FollowParent::new(1.25));
    goals.add(5, bear::FindFood);
    goals.add(6, bear::HarvestFood::new());
    goals.add(6, bear::SniffFlowers::new());
    goals.add(7, bear::SitAndEat::default());
    goals.add(8, Stroll::new(1.0));
    goals.add(9, LookAtPlayer::new(6.0));
    goals.add(10, RandomLookAround::default());
    let targets = &mut mob.ai.targets;
    targets.add(1, HurtByTarget::new().polar_bear().alert_if(same_species));
    targets.add(2, HurtByTarget::new().alert().alert_if(same_species));
    targets.add(2, NearestTarget::player(false).reach().interval(20).follow_scale(0.5).when(bear::protects_cubs));
    targets.add(3, NearestTarget::player(true).when(bear::angry_player));
    targets.add(3, NearestTarget::player(true).interval(20).when(|mob, world| {
        !mob.extra.tame && !mob.is_baby() && !bear::state(&mob.ai.nav).sleeping
            && (!world.daylight || mob_brightness(mob, world) < 0.5)
    }));
    targets.add(4, NearestTarget::new(Prey::Filter(bear::natural_prey), true).when(|mob, _| {
        let pose = bear::state(&mob.ai.nav);
        !mob.is_baby() && !pose.sleeping && pose.hunting <= 0
    }));
    targets.add(5, super::golem::ResetUniversalAnger::default());
}
