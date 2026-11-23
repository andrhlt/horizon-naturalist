fn naturalist_species_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_core::nbt::Tag;
    use std::ops::ControlFlow;
    let Some(index) = game.mob_interaction_target(target) else { return ControlFlow::Continue(()) };
    let kind = game.mobs[index].kind.id.to_string();
    let item = game.held().map(|stack| stack.item.to_string()).unwrap_or_default();
    let actor = game.acting_player_uuid();
    let raw_owner = game.persist.raw.get(&game.mobs[index].id).and_then(|tag| tag.get("Owner"))
        == Some(&Tag::IntArray(actor.to_vec()));
    let owner = game.mobs[index].extra.tame
        && (game.mobs[index].extra.owner == Some(actor) || raw_owner);
    if kind != "naturalist:komodo_dragon" && kind != "naturalist:tortoise" { return ControlFlow::Continue(()); }
    if kind == "naturalist:tortoise" && !owner { return ControlFlow::Continue(()); }
    let food = game.mobs[index].kind.food.iter().any(|candidate| candidate == &item);
    if !food { return ControlFlow::Continue(()); }
    if game.mobs[index].health < game.mobs[index].attrs.max_health {
        game.mobs[index].heal(&game.loop_context, 2.0);
    } else {
        let id = game.mobs[index].id;
        let actor = game.acting_player_uuid();
        game.mobs[index].extra.in_love = 600;
        game.mobs[index].extra.love_player = true;
        game.mobs[index].extra.love_cause = Some(actor);
        game.love_cause.insert(id, game.lan_acting);
        game.native_entity_enter_love_mode(id, &kind, game.lan_acting_uuid);
        game.mob_event_particles(id, 18);
    }
    game.consume_held();
    ControlFlow::Break(Ok(true))
}
