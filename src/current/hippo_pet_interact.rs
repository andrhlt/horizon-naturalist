fn hippo_pet_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::{actions::Action, text::Text};
    use horizon_core::nbt::Tag;
    use mod_api::{Host, SoundSource};
    use std::ops::ControlFlow;

    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let mob = &game.mobs[index];
    if mob.kind.id != "naturalist:hippo" || !mob.alive() {
        return ControlFlow::Continue(());
    }
    let id = mob.id;
    let actor = game.acting_player_uuid();
    let item = game
        .held()
        .map(|stack| stack.item.to_string())
        .unwrap_or_default();
    let empty_hand = item.is_empty() || item == "minecraft:air";
    let melon = item == "minecraft:melon";
    let baby = game.mobs[index].is_baby();
    let saved = game.persist.raw.get(&id);
    let saved_tame = saved
        .and_then(|tag| tag.get("Tame"))
        .and_then(|tag| match tag {
            Tag::Byte(value) => Some(*value != 0),
            _ => None,
        })
        .unwrap_or(false);
    let saved_owner_matches = saved
        .and_then(|tag| tag.get("Owner"))
        .and_then(|tag| match tag {
            Tag::IntArray(value) => Some(value.as_slice() == actor.as_slice()),
            _ => None,
        })
        .unwrap_or(false);
    let tamed = game.mobs[index].extra.tame || saved_tame;
    let owner = tamed && (game.mobs[index].extra.owner == Some(actor) || saved_owner_matches);

    if baby && !tamed && melon {
        use horizon_core::random::RandomSource;
        game.consume_held();
        let success = game.mobs[index].random.next_int_bound(3).unwrap_or(1) == 0;
        if success {
            game.mobs[index].extra.tame = true;
            game.mobs[index].extra.owner = Some(actor);
            game.mobs[index].extra.sitting = true;
            horizon_client::mobs::tame::restore_sitting(&mut game.mobs[index], true);
            let raw = game
                .persist
                .raw
                .entry(id)
                .or_insert_with(|| Tag::Compound(Default::default()));
            if let Tag::Compound(fields) = raw {
                fields.insert("Owner".into(), Tag::IntArray(actor.to_vec()));
                fields.insert("Tame".into(), Tag::Byte(1));
                fields.insert("Sitting".into(), Tag::Byte(1));
                fields.insert("FollowingOwner".into(), Tag::Byte(1));
            }
            let view = game.mob_entity(&game.mobs[index]);
            game.trigger_entity("minecraft:tame_animal", "entity", &view);
        }
        game.mob_event_particles(id, if success { 7 } else { 6 });
        return ControlFlow::Break(Ok(true));
    }

    if owner && melon && game.mobs[index].health < game.mobs[index].attrs.max_health {
        game.consume_held();
        game.mobs[index].heal(&game.loop_context, 4.0);
        return ControlFlow::Break(Ok(true));
    }

    // Adults carry the offered stack while entering love mode. Transfer the
    // stack from the player so this visual behavior cannot duplicate items.
    if !baby && melon && game.mobs[index].extra.main_hand_stack().is_none() {
        let Some(carried) = game.held().cloned() else {
            return ControlFlow::Continue(());
        };
        let fed = horizon_client::mobs::ai::feed_animal(
            &mut game.mobs[index],
            &mut game.world.random,
            Some(actor),
        )
        .is_some();
        game.mobs[index].extra.set_main_hand_stack(Some(carried));
        if !game.creative {
            *game.held_mut() = None;
        }
        let raw = game
            .persist
            .raw
            .entry(id)
            .or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw {
            fields.insert("NaturalistHippoCarryTimer".into(), Tag::Int(11));
        }
        if fed && game.mobs[index].extra.in_love > 0 {
            game.love_cause.insert(id, game.lan_acting);
            game.native_entity_enter_love_mode(id, "naturalist:hippo", game.lan_acting_uuid);
            game.mob_event_particles(id, 18);
        }
        return ControlFlow::Break(Ok(true));
    }

    if !owner {
        return ControlFlow::Continue(());
    }

    const SIT_ACTION: &str = "naturalist:hippo_sit";
    if empty_hand && game.keys.held(Action::Sneak) {
        if game.cooldowns.is_on_cooldown(SIT_ACTION) {
            return ControlFlow::Break(Ok(true));
        }
        let sitting = !game.mobs[index].extra.sitting;
        game.mobs[index].extra.sitting = sitting;
        horizon_client::mobs::tame::restore_sitting(&mut game.mobs[index], sitting);
        game.mobs[index].nav_stop();
        let raw = game
            .persist
            .raw
            .entry(id)
            .or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw {
            fields.insert("Sitting".into(), Tag::Byte(i8::from(sitting)));
        }
        game.add_cooldown(std::sync::Arc::from(SIT_ACTION), 5);
        return ControlFlow::Break(Ok(true));
    }

    if item != "naturalist:whistle"
        || game.keys.held(Action::Sneak)
        || game.cooldowns.is_on_cooldown(&item)
    {
        return ControlFlow::Continue(());
    }
    let following = game
        .persist
        .raw
        .get(&id)
        .and_then(|tag| tag.get("FollowingOwner"))
        .and_then(horizon_world::datafix::dynamic::as_number)
        .is_none_or(|value| value.int_value() != 0);
    let at = game.mobs[index].center();
    game.mobs[index].extra.sitting = false;
    horizon_client::mobs::tame::restore_sitting(&mut game.mobs[index], false);
    if following {
        game.mobs[index].ai.target = None;
    }
    let raw = game
        .persist
        .raw
        .entry(id)
        .or_insert_with(|| Tag::Compound(Default::default()));
    if let Tag::Compound(fields) = raw {
        fields.insert("FollowingOwner".into(), Tag::Byte(i8::from(!following)));
        fields.insert("Sitting".into(), Tag::Byte(0));
    }
    game.add_cooldown(std::sync::Arc::from(item), 20);
    if let Some(level) = game.level(game.dim.id()) {
        level.play_sound(
            game,
            "naturalist:item.whistle",
            at,
            SoundSource::Neutral,
            0.8,
            1.0,
        );
    }
    let mode = Text::translate(
        if !following {
            "naturalist.whistle.following"
        } else {
            "naturalist.whistle.wandering"
        },
        &[],
    );
    game.send_overlay_message(Text::translate(
        "naturalist.whistle.message",
        &[Text::literal("Hippo"), mode],
    ));
    ControlFlow::Break(Ok(true))
}
