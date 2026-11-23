fn giraffe_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_core::nbt::Tag;
    use std::ops::ControlFlow;

    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    if game.mobs[index].kind.id != "naturalist:giraffe" || !game.mobs[index].alive() {
        return ControlFlow::Continue(());
    }

    let actor = game.acting_player_uuid();
    let id = game.mobs[index].id;
    let item = game.held().map(|stack| stack.item.to_string()).unwrap_or_default();
    let food = game.items().item_tags.get("naturalist:giraffe_food_items")
        .is_some_and(|tag| tag.contains(item.as_str()));
    let baby = game.mobs[index].is_baby();
    let raw_pet = game.persist.raw.get(&id);
    let saved_tame = raw_pet.and_then(|tag| tag.get("Tame"))
        .is_some_and(|value| matches!(value, Tag::Byte(flag) if *flag != 0));
    let saved_owner = raw_pet.and_then(|tag| tag.get("Owner"))
        .and_then(|value| match value { Tag::IntArray(words) if words.as_slice() == actor => Some(()), _ => None })
        .is_some();
    let tame = game.mobs[index].extra.tame || saved_tame;
    let owner = tame && (game.mobs[index].extra.owner == Some(actor) || saved_owner);

    if !tame && baby && food {
        if !game.creative { game.consume_held(); }
        let success = game.mobs[index].random.next_int_bound(3).unwrap_or(1) == 0;
        if success {
            game.mobs[index].extra.tame = true;
            game.mobs[index].extra.owner = Some(actor);
            let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
            if let Tag::Compound(fields) = raw {
                fields.insert("Tame".into(), Tag::Byte(1));
                fields.insert("Owner".into(), Tag::IntArray(actor.to_vec()));
            }
            let view = game.mob_entity(&game.mobs[index]);
            game.trigger_entity("minecraft:tame_animal", "entity", &view);
        }
        game.mob_event_particles(id, if success { 7 } else { 6 });
        return ControlFlow::Break(Ok(true));
    }

    if food && owner && game.mobs[index].health < game.mobs[index].attrs.max_health {
        if !game.creative { game.consume_held(); }
        game.mobs[index].heal(&game.loop_context, 4.0);
        return ControlFlow::Break(Ok(true));
    }

    if food {
        if let Some(_outcome) = horizon_client::mobs::ai::feed_animal(
            &mut game.mobs[index], &mut game.world.random, Some(actor),
        ) {
            if !game.creative { game.consume_held(); }
            if game.mobs[index].extra.in_love > 0 {
                game.love_cause.insert(id, game.lan_acting);
                game.native_entity_enter_love_mode(id, "naturalist:giraffe", game.lan_acting_uuid);
                game.mob_event_particles(id, 18);
            }
            return ControlFlow::Break(Ok(true));
        }
    }

    if owner && !baby && item.is_empty() {
        if game.native_may_board(id) {
            game.start_riding(id);
            // Java's Giraffe.getControllingPassenger accepts its owner without
            // a saddle. Horizon's equine rider path gates movement on this
            // flag, so mirror that control state while the passenger is aboard.
            game.mobs[index].extra.set_saddled(true);
            return ControlFlow::Break(Ok(true));
        }
    }

    ControlFlow::Continue(())
}
