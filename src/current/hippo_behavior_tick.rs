fn hippo_behavior_tick(game: &mut Game<'_>, entity: u32, kind: &str) {
    use mod_api::Host;
    if kind != "naturalist:hippo" {
        return;
    }

    crate::game::naturalist_hippo_boat::refresh(
        game.boats
            .iter()
            .filter(|boat| !boat.chest && &*boat.wood != "bamboo")
            .map(|boat| (boat.id, boat.position))
            .collect(),
    );

    let Some(entity_ref) = game.entity(entity) else {
        return;
    };
    let Some(handle) = game.mob(entity_ref) else {
        return;
    };
    handle.remove_goal(game, "BreedGoal");

    let Some(index) = game.mobs.iter().position(|mob| mob.id == entity) else {
        return;
    };
    if !game.mobs[index].ai.goals.has("NaturalistHippoBreed") {
        game.mobs[index].ai.goals.add(
            6,
            crate::game::naturalist_hippo_breed::HippoBreed::default(),
        );
    }
    if !game.mobs[index].ai.goals.has("NaturalistHippoBoatAttack") {
        game.mobs[index].ai.goals.add(
            4,
            crate::game::naturalist_hippo_boat::HippoBoatAttack::default(),
        );
    }

    let attack_damage = game.mobs[index].attrs.attack_damage;
    for boat_id in crate::game::naturalist_hippo_boat::take_attacks(entity) {
        if let Some(boat_index) = game
            .boats
            .iter()
            .position(|boat| boat.id == boat_id && !boat.chest && &*boat.wood != "bamboo")
        {
            game.hurt_boat(boat_index, attack_damage, false);
        }
    }
}
