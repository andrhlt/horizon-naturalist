fn capybara_pet_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::{actions::Action, text::Text};
    use horizon_core::nbt::Tag;
    use mod_api::{Host, SoundSource};
    use std::ops::ControlFlow;

    let Some(index) = game.mob_interaction_target(target) else { return ControlFlow::Continue(()) };
    let mob = &game.mobs[index];
    if mob.kind.id != "naturalist:capybara" || !mob.alive() { return ControlFlow::Continue(()) }
    let id = mob.id;
    // The engine helper resolves the active LAN guest when a guest packet is
    // being replayed and the host player otherwise.
    let actor = game.acting_player_uuid();
    let item = game.held().map(|stack| stack.item.to_string()).unwrap_or_default();
    let empty_hand = item.is_empty() || item == "minecraft:air";
    let food = item == "minecraft:melon_slice";
    let saved_tame = game.persist.raw.get(&id).and_then(|tag| tag.get("Tame")).and_then(|tag| match tag {
        Tag::Byte(value) => Some(*value != 0),
        _ => None,
    }).unwrap_or(false);
    let saved_owner_matches = game.persist.raw.get(&id).and_then(|tag| tag.get("Owner")).and_then(|tag| match tag {
        Tag::IntArray(value) => Some(value.as_slice() == actor.as_slice()),
        _ => None,
    }).unwrap_or(false);
    let owner = (game.mobs[index].extra.tame || saved_tame)
        && (game.mobs[index].extra.owner == Some(actor) || saved_owner_matches);
    let tamed = game.mobs[index].extra.tame || saved_tame;

    if !owner && !tamed && food {
        use horizon_core::random::RandomSource;
        game.consume_held();
        let success = game.mobs[index].random.next_int_bound(3).unwrap_or(1) == 0;
        if success {
            game.mobs[index].extra.tame = true;
            game.mobs[index].extra.owner = Some(actor);
            game.mobs[index].extra.color = 14;
            let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
            if let Tag::Compound(fields) = raw {
                fields.insert("Owner".into(), Tag::IntArray(actor.to_vec()));
                fields.insert("Tame".into(), Tag::Byte(1));
                fields.insert("DyeColor".into(), Tag::Int(14));
                fields.insert("FollowingOwner".into(), Tag::Byte(1));
            }
            let view = game.mob_entity(&game.mobs[index]);
            game.trigger_entity("minecraft:tame_animal", "entity", &view);
        }
        game.mob_event_particles(id, if success { 7 } else { 6 });
        return ControlFlow::Break(Ok(true));
    }
    if owner && food && game.mobs[index].health < game.mobs[index].attrs.max_health {
        game.consume_held();
        game.mobs[index].heal(&game.loop_context, 2.0);
        return ControlFlow::Break(Ok(true));
    }
    const SIT_ACTION: &str = "naturalist:capybara_sit";
    if empty_hand && owner && game.cooldowns.is_on_cooldown(SIT_ACTION) {
        return ControlFlow::Break(Ok(true));
    }
    if food && let Some(_) = horizon_client::mobs::ai::feed_animal(&mut game.mobs[index], &mut game.world.random, Some(actor)) {
        game.consume_held();
        let mob = &game.mobs[index];
        if mob.extra.in_love > 0 {
            game.love_cause.insert(id, game.lan_acting);
            game.native_entity_enter_love_mode(id, "naturalist:capybara", game.lan_acting_uuid);
            game.mob_event_particles(id, 18);
        }
        return ControlFlow::Break(Ok(true));
    }
    if !owner || !(empty_hand || item == "naturalist:whistle" && !game.keys.held(Action::Sneak)) {
        return ControlFlow::Continue(());
    }
    if empty_hand {
        // The live runtime flag reflects command updates and the loaded mob
        // state; raw NBT may still hold the prior persisted value.
        let sitting = !game.mobs[index].extra.sitting;
        horizon_client::mobs::tame::restore_sitting(&mut game.mobs[index], sitting);
        game.mobs[index].extra.sitting = sitting;
        game.mobs[index].nav_stop();
        let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw { fields.insert("Sitting".into(), Tag::Byte(i8::from(sitting))); }
        // One physical right-click can reach this hook through more than one
        // input path. Debounce the empty-hand toggle like a normal item use.
        game.add_cooldown(std::sync::Arc::from(SIT_ACTION), 5);
        return ControlFlow::Break(Ok(true));
    }
    if game.cooldowns.is_on_cooldown(&item) { return ControlFlow::Continue(()) }
    let following = game.persist.raw.get(&id).and_then(|tag| tag.get("FollowingOwner"))
        .and_then(horizon_world::datafix::dynamic::as_number).is_none_or(|n| n.int_value() != 0);
    let at = game.mobs[index].center();
    horizon_client::mobs::tame::restore_sitting(&mut game.mobs[index], false);
    game.mobs[index].extra.sitting = false;
    if following { game.mobs[index].ai.target = None; }
    let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
    if let Tag::Compound(fields) = raw {
        fields.insert("FollowingOwner".into(), Tag::Byte(i8::from(!following)));
        fields.insert("Sitting".into(), Tag::Byte(0));
    }
    game.add_cooldown(std::sync::Arc::from(item), 20);
    if let Some(level) = game.level(game.dim.id()) {
        level.play_sound(game, "naturalist:item.whistle", at, SoundSource::Neutral, 0.8, 1.0);
    }
    let mode = Text::translate(if !following { "naturalist.whistle.following" } else { "naturalist.whistle.wandering" }, &[]);
    game.send_overlay_message(Text::translate("naturalist.whistle.message", &[Text::literal("Capybara"), mode]));
    ControlFlow::Break(Ok(true))
}
