fn deer_predator_tags(game: &mut Game<'_>) {
    use horizon_client::mobs::{nav::naturalist_deer_predator, drops::is_animal};

    let tags = game
        .world_tables
        .as_ref()
        .and_then(|tables| tables.pack_tags.get("entity_type"))
        .unwrap_or(&game.res.mob_data().entity_tags);
    let predators = tags.get(naturalist_deer_predator::TAG);
    let turkey_hostiles = tags.get(naturalist_deer_predator::TURKEY_HOSTILES);
    use horizon_client::mobs::naturalist_scorpion;
    let scorpion_prey = tags.get(naturalist_scorpion::PREY);
    use horizon_client::mobs::naturalist_komodo;
    let komodo_prey = tags.get(naturalist_komodo::PREY);
    use horizon_client::mobs::naturalist_bears;
    let bear_prey = tags.get(naturalist_bears::PREY);
    use horizon_client::mobs::nav::naturalist_frog_food;
    let frog_food = tags.get("minecraft:frog_food");
    use horizon_client::mobs::goals::{naturalist_cat_goals as cats, naturalist_lion_goals as lion};
    let tiger_prey = tags.get(cats::PREY);
    let lion_prey = tags.get(lion::PREY);
    use horizon_client::mobs::nav::naturalist_shark_state as shark;
    let shark_prey = tags.get(shark::PREY);
    use horizon_client::mobs::goals::naturalist_snake_goals as snake;
    let snake_prey = tags.get(snake::PREY);
    for mob in &mut game.mobs {
        shark::set_prey(&mut mob.ai.nav, shark_prey.is_some_and(|types| types.contains(&mob.kind.id)));
        let tiger_prey = tiger_prey.is_some_and(|types| types.contains(&mob.kind.id));
        let lion_prey = lion_prey.is_some_and(|types| types.contains(&mob.kind.id));
        cats::mark_prey(mob, tiger_prey, lion_prey);
        snake::state_mut(&mut mob.ai.nav).prey =
            snake_prey.is_some_and(|types| types.contains(&mob.kind.id));
        naturalist_frog_food::set(
            &mut mob.ai.nav,
            frog_food.is_some_and(|types| types.contains(&mob.kind.id)),
        );
        naturalist_bears::state_mut(&mut mob.ai.nav).prey =
            bear_prey.is_some_and(|types| types.contains(&mob.kind.id));
        naturalist_komodo::set_prey(&mut mob.ai.nav,
            komodo_prey.is_some_and(|types| types.contains(&mob.kind.id)));
        naturalist_scorpion::set_prey(&mut mob.ai.nav,
            scorpion_prey.is_some_and(|types| types.contains(&mob.kind.id)));
        naturalist_deer_predator::set_turkey_hostile(
            &mut mob.ai.nav,
            turkey_hostiles.is_some_and(|types| types.contains(&mob.kind.id)),
        );
        naturalist_deer_predator::set(
            &mut mob.ai.nav,
            predators.is_some_and(|types| is_animal(mob.kind.species) && types.contains(&mob.kind.id)),
        );
    }
}
