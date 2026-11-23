fn crab_tame_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_core::nbt::Tag;
    use std::ops::ControlFlow;

    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    if game.mobs[index].kind.id != "naturalist:crab" || !game.mobs[index].alive() {
        return ControlFlow::Continue(());
    }
    let item = game.held().map(|stack| stack.item.as_ref()).unwrap_or("");
    if item != "minecraft:tropical_fish" {
        return ControlFlow::Continue(());
    }

    let actor = game.acting_player_uuid();
    let mob = &game.mobs[index];
    if mob.extra.tame {
        if mob.extra.owner == Some(actor) && mob.health < mob.attrs.max_health {
            if !game.creative {
                game.consume_held();
            }
            game.mobs[index].heal(&game.loop_context, 2.0);
            return ControlFlow::Break(Ok(true));
        }
        return ControlFlow::Continue(());
    }

    if !game.creative {
        game.consume_held();
    }
    let tame = game.mobs[index].random.next_int_bound(3).unwrap_or(1) == 0;
    let id = game.mobs[index].id;
    if tame {
        // The shared Horizon helper applies Wolf health defaults to non-Wolf
        // species; Java Crab remains at its registered 8 health.
        game.mobs[index].extra.tame = true;
        game.mobs[index].extra.owner = Some(actor);
        game.mobs[index].ai.target = None;
        horizon_client::mobs::tame::restore_sitting(&mut game.mobs[index], false);
        let raw = game
            .persist
            .raw
            .entry(id)
            .or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw {
            fields.insert("Tame".into(), Tag::Byte(1));
            fields.insert("Owner".into(), Tag::IntArray(actor.to_vec()));
            fields.insert("FollowingOwner".into(), Tag::Byte(1));
            fields.insert("Sitting".into(), Tag::Byte(0));
        }
        let at = game.mobs[index].center();
        let source = game.who_context(Some(horizon_client::mobs::Who::Player), None);
        game.post_game_event("minecraft:tame_animal", at, source);
    }
    game.mob_event_particles(id, if tame { 7 } else { 6 });
    ControlFlow::Break(Ok(true))
}
