fn deer_goals(mob: &mut Mob) {
    use super::goals::{Avoid, AvoidEntity, Breed, EatBlock, Float, FollowParent, LookAtPlayer,
        Panic, RandomLookAround, Stroll, Tempt};
    use std::sync::Arc;
    if mob.kind.id != "naturalist:deer" {
        return;
    }
    let kind = Arc::make_mut(&mut mob.kind);
    let apples: Arc<[String]> = Arc::from(vec!["minecraft:apple".to_owned()]);
    kind.food = apples.clone();
    kind.tempt = apples.clone();
    let goals = &mut mob.ai.goals;
    goals.clear();
    goals.add(0, Float);
    goals.add(1, Panic::new(1.6));
    goals.add(2, Breed::new(1.0));
    goals.add(3, Tempt::new(1.25, apples, true));
    goals.add(4, AvoidEntity::new(Avoid::Player, 8.0, 1.5, 2.0)
        .player_when(|_, player| !player.creative && !player.spectator && !player.sneaking));
    let monsters = super::species::ALL.iter()
        .map(|definition| definition.species)
        .filter(|&species| super::drops::is_monster(species))
        .collect();
    goals.add(4, AvoidEntity::new(Avoid::Species(monsters), 4.0, 1.5, 2.0));
    let animals = super::species::ALL.iter()
        .map(|definition| definition.species)
        .filter(|&species| super::drops::is_animal(species))
        .collect();
    goals.add(
        4,
        AvoidEntity::new(Avoid::Species(animals), 10.0, 1.5, 2.0).seen_when(|seen| {
            seen.effects.contains(&super::nav::naturalist_deer_predator::TAG)
        }),
    );
    goals.add(5, FollowParent::new(1.5));
    goals.add(6, EatBlock::default());
    goals.add(7, Stroll::avoiding_water(1.0));
    goals.add(8, LookAtPlayer::new(6.0));
    goals.add(9, RandomLookAround::default());
}
