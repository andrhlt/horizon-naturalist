fn butterfly_flower_interact(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use std::ops::ControlFlow;
    let Some(index) = game.mob_interaction_target(target) else { return ControlFlow::Continue(()) };
    if game.mobs[index].kind.id != "naturalist:butterfly" || !game.mobs[index].alive() { return ControlFlow::Continue(()) }
    let Some(stack) = game.held().cloned() else { return ControlFlow::Continue(()) };
    if !game.items().item_tags.get("minecraft:flowers").is_some_and(|tag| tag.contains(stack.item.as_ref())) { return ControlFlow::Continue(()) }
    if game.mobs[index].extra.age != 0 || game.mobs[index].extra.in_love > 0 { return ControlFlow::Continue(()) }
    let actor = game.acting_player_uuid();
    let id = game.mobs[index].id;
    game.mobs[index].extra.in_love = 600;
    game.mobs[index].extra.love_player = true;
    game.mobs[index].extra.love_cause = Some(actor);
    game.love_cause.insert(id, game.lan_acting);
    game.native_entity_enter_love_mode(id, "naturalist:butterfly", game.lan_acting_uuid);
    game.mob_event_particles(id, 18);
    let entity = game.mob_entity(&game.mobs[index]);
    game.trigger_item_entity("minecraft:player_interacted_with_entity", &stack, &entity);
    game.consume_held();
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
