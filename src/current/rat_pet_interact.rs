fn rat_pet_interact(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::{actions::Action, text::Text, first_person::Hand};
    use horizon_core::nbt::Tag;
    use mod_api::{Host, SoundSource};
    use std::ops::ControlFlow;
    let Some(index) = game.mob_interaction_target(target) else { return ControlFlow::Continue(()) };
    if game.mobs[index].kind.id != "naturalist:rat" || !game.mobs[index].alive() { return ControlFlow::Continue(()) }
    let id = game.mobs[index].id;
    let actor = game.acting_player_uuid();
    let item = game.held().map(|stack| stack.item.to_string()).unwrap_or_default();
    let owner = game.mobs[index].extra.tame && game.mobs[index].extra.owner == Some(actor);
    // Java's Rat mobInteract heals an injured owner-tamed Rat with its resolved food tag.
    let food = item == "minecraft:bread";
    if !owner && !game.mobs[index].extra.tame && food {
        use horizon_core::random::RandomSource;
        game.consume_held();
        let success = game.mobs[index].random.next_int_bound(10).unwrap_or(9) < 7;
        if success {
            game.mobs[index].extra.tame = true;
            game.mobs[index].extra.owner = Some(actor);
            horizon_client::mobs::tame::restore_sitting(&mut game.mobs[index], true);
            let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
            if let Tag::Compound(fields) = raw { fields.insert("Owner".into(), Tag::IntArray(actor.to_vec())); fields.insert("Sitting".into(), Tag::Byte(1)); fields.insert("FollowingOwner".into(), Tag::Byte(1)); }
            let view = game.mob_entity(&game.mobs[index]);
            game.trigger_entity("minecraft:tame_animal", "entity", &view);
        }
        game.mob_event_particles(id, if success { 7 } else { 6 });
        return ControlFlow::Break(Ok(true));
    }
    if game.mobs[index].extra.tame && food && game.mobs[index].health < game.mobs[index].attrs.max_health {
        game.consume_held();
        game.mobs[index].heal(&game.loop_context, 2.0);
        return ControlFlow::Break(Ok(true));
    }
    let whistle_usable = item == "naturalist:whistle" && !game.keys.held(Action::Sneak) && !game.cooldowns.is_on_cooldown(&item);
    let claimed = game.lan_acting_uuid
        .and_then(|uuid| game.player(uuid))
        .and_then(|player| game.player_announced_channels(player))
        .is_some_and(|channels| channels.iter().any(|c| c == "naturalist:rat_pet_atomic_v1"));
    // Java Animal.mobInteract consumes an adult food interaction client-side even
    // while positive breeding age makes the server's breeding action pass.
    if food && game.mobs[index].extra.tame && game.mobs[index].extra.age > 0 {
        return ControlFlow::Break(Ok(true));
    }
    if food && claimed && let Some(crate::game::GuestUse::Entity(Hand::Main, target)) = game.guest_use {
        let previous = game.guest_use;
        game.guest_use = Some(crate::game::GuestUse::Item(Hand::Main));
        let main = game.use_item();
        if !matches!(main, Ok(None)) { game.guest_use = previous; return match main { Ok(_) => ControlFlow::Break(Ok(true)), Err(e) => ControlFlow::Break(Err(e)) }; }
        game.guest_use = Some(crate::game::GuestUse::Entity(Hand::Off, target));
        let off = game.use_item();
        if !matches!(off, Ok(None)) { game.guest_use = previous; return match off { Ok(_) => ControlFlow::Break(Ok(true)), Err(e) => ControlFlow::Break(Err(e)) }; }
        game.guest_use = Some(crate::game::GuestUse::Item(Hand::Off));
        let result = game.use_item();
        game.guest_use = previous;
        return match result { Ok(_) => ControlFlow::Break(Ok(true)), Err(e) => ControlFlow::Break(Err(e)) };
    }
    if !(owner && (!food && !game.keys.held(Action::Sneak))) {
        // A matching client optimistically ends a main-hand click. If Java's
        // server result passes, perform the offhand turn once in this packet.
        if claimed && !game.keys.held(Action::Sneak)
            && let Some(crate::game::GuestUse::Entity(Hand::Main, target)) = game.guest_use
        {
            let previous = game.guest_use;
            game.guest_use = Some(crate::game::GuestUse::Item(Hand::Main));
            let main = game.use_item();
            if !matches!(main, Ok(None)) { game.guest_use = previous; return match main { Ok(_) => ControlFlow::Break(Ok(true)), Err(e) => ControlFlow::Break(Err(e)) }; }
            game.guest_use = Some(crate::game::GuestUse::Entity(Hand::Off, target));
            let result = game.use_item();
            if matches!(result, Ok(None)) {
                game.guest_use = Some(crate::game::GuestUse::Item(Hand::Off));
                let fallback = game.use_item();
                game.guest_use = previous;
                return match fallback { Ok(_) => ControlFlow::Break(Ok(true)), Err(e) => ControlFlow::Break(Err(e)) };
            }
            game.guest_use = previous;
            return match result { Ok(_) => ControlFlow::Break(Ok(true)), Err(e) => ControlFlow::Break(Err(e)) };
        }
        return ControlFlow::Continue(());
    }
    if !whistle_usable {
        let sitting = game.mobs[index].extra.sitting;
        horizon_client::mobs::tame::restore_sitting(&mut game.mobs[index], !sitting);
        game.mobs[index].nav_stop();
        let raw = game.persist.raw.entry(id).or_insert_with(||Tag::Compound(Default::default()));
        if let Tag::Compound(fields)=raw { fields.insert("Sitting".into(), Tag::Byte(i8::from(!sitting))); }
        return ControlFlow::Break(Ok(true));
    }
    if item != "naturalist:whistle" || game.keys.held(Action::Sneak) || game.cooldowns.is_on_cooldown(&item) { return ControlFlow::Continue(()) }
    let following = game.persist.raw.get(&id).and_then(|tag|tag.get("FollowingOwner"))
        .and_then(horizon_world::datafix::dynamic::as_number).is_none_or(|n|n.int_value()!=0);
    let at = game.mobs[index].center();
    horizon_client::mobs::tame::restore_sitting(&mut game.mobs[index],false);
    if following { game.mobs[index].ai.target = None; }
    let raw = game.persist.raw.entry(id).or_insert_with(||Tag::Compound(Default::default()));
    if let Tag::Compound(fields)=raw {
        fields.insert("FollowingOwner".into(), Tag::Byte(i8::from(!following)));
        fields.insert("Sitting".into(), Tag::Byte(0));
    }
    game.add_cooldown(std::sync::Arc::from(item),20);
    if let Some(level)=game.level(game.dim.id()) { level.play_sound(game,"naturalist:item.whistle",at,SoundSource::Neutral,0.8,1.0); }
    let name=game.mobs[index].custom_name.as_deref().unwrap_or("Rat").to_owned();
    let mode=Text::translate(if !following {"naturalist.whistle.following"} else {"naturalist.whistle.wandering"}, &[]);
    game.send_overlay_message(Text::translate("naturalist.whistle.message", &[Text::literal(name),mode]));
    ControlFlow::Break(Ok(true))
}
