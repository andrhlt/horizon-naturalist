fn land_goals(mob: &mut Mob) {
    use super::goals::{
        Avoid, AvoidEntity, Breed, Float, FollowParent, HurtByTarget, LookAtPlayer, MeleeAttack,
        NearestTarget, Panic, Prey, RandomLookAround, Stroll, Tempt,
    };
    use std::sync::Arc;

    let boar = match mob.kind.id.as_str() {
        "naturalist:boar" => true,
        "naturalist:turkey" => false,
        _ => return,
    };
    let foods: &[&str] = if boar {
        &["beetroot", "carrot", "potato"]
    } else {
        &["beetroot_seeds", "melon_seeds", "pitcher_pod", "pumpkin_seeds", "torchflower_seeds", "wheat_seeds"]
    };
    let food: Arc<[String]> = foods.iter().map(|food| format!("minecraft:{food}")).collect();
    let kind = Arc::make_mut(&mut mob.kind);
    kind.food = food.clone();
    kind.tempt = food.clone();

    let goals = &mut mob.ai.goals;
    goals.clear();
    mob.ai.targets.clear();
    fn holds_food(mob: &Mob, player: &super::PlayerView) -> bool {
        [&player.main_hand, &player.off_hand].into_iter().flatten()
            .any(|stack| mob.kind.food.iter().any(|food| food.as_str() == stack.as_ref()))
    }
    goals.add(0, Float);
    if boar {
        goals.add(1, Breed::new(1.0));
        goals.add(2, Tempt::new(1.2, food, false));
        goals.add(3, AvoidEntity::new(Avoid::Player, 16.0, 1.5, 1.5)
            .when(|mob| !mob.is_baby() && mob.health <= mob.attrs.max_health * 0.5)
            .player_when(|mob, player| !holds_food(mob, player)));
        goals.add(4, MeleeAttack::new(1.2, false));
        goals.add(5, Panic::new(1.25));
        goals.add(6, FollowParent::new(1.1));
        mob.ai.targets.add(1, HurtByTarget::new());
        mob.ai.targets.add(2, NearestTarget::player(true).when(|mob, world| {
            mob.extra.anger > 0
                && (mob.extra.angry_at == Some(super::Who::Player)
                    || (world.universal_anger && mob.extra.angry_at.is_none()))
        }));
        mob.ai.targets.add(3, super::golem::ResetUniversalAnger::default());
    } else {
        goals.add(1, Panic::new(1.5));
        goals.add(2, AvoidEntity::new(Avoid::Player, 6.0, 1.5, 2.0)
            .player_when(|mob, player| !player.creative && !player.spectator && !player.sneaking
                && !holds_food(mob, player)));
        goals.add(3, Breed::new(1.0));
        goals.add(4, Tempt::new(1.0, food, false));
        goals.add(5, FollowParent::new(1.1));
        goals.add(6, MeleeAttack::new(1.0, true));
        mob.ai.targets.add(1, NearestTarget::new(
            Prey::Filter(|seen| seen.effects.contains(&super::nav::naturalist_deer_predator::TURKEY_HOSTILES)), true,
        ));
    }
    goals.add(7, Stroll::avoiding_water(1.0));
    goals.add(8, LookAtPlayer::new(6.0));
    goals.add(9, RandomLookAround::default());
}
