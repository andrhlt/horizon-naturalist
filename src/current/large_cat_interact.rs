use horizon_client::game::Game;

fn large_cat_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_core::nbt::Tag;
    use std::ops::ControlFlow;

    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let kind = game.mobs[index].kind.id.to_string();
    if !matches!(kind.as_str(), "naturalist:lion" | "naturalist:tiger") || !game.mobs[index].alive() {
        return ControlFlow::Continue(());
    }
    let id = game.mobs[index].id;
    let actor = game.acting_player_uuid();
    let item = game.held().map(|stack| stack.item.to_string()).unwrap_or_default();
    let food = matches!(item.as_str(),
        "minecraft:beef" | "minecraft:chicken" | "minecraft:cooked_beef" |
        "minecraft:cooked_chicken" | "minecraft:cooked_mutton" | "minecraft:cooked_porkchop" |
        "minecraft:cooked_rabbit" | "minecraft:mutton" | "minecraft:porkchop" |
        "minecraft:rabbit" | "naturalist:bushmeat" | "naturalist:clam_meat" |
        "naturalist:cooked_bushmeat" | "naturalist:cooked_clam_meat" |
        "naturalist:cooked_crab_meat" | "naturalist:cooked_drumstick" |
        "naturalist:cooked_duck" | "naturalist:cooked_lizard_tail" |
        "naturalist:cooked_mammoth_meat" | "naturalist:cooked_venison" |
        "naturalist:crab_meat" | "naturalist:drumstick" | "naturalist:duck" |
        "naturalist:lizard_tail" | "naturalist:mammoth_meat" | "naturalist:morsel" |
        "naturalist:venison");
    let saved = game.persist.raw.get(&id);
    let saved_tame = saved.and_then(|tag| tag.get("Tame"))
        .is_some_and(|value| matches!(value, Tag::Byte(flag) if *flag != 0));
    let saved_owner = saved.and_then(|tag| tag.get("Owner"))
        .and_then(|value| match value {
            Tag::IntArray(words) if words.as_slice() == actor => Some(()),
            _ => None,
        }).is_some();
    let tame = game.mobs[index].extra.tame || saved_tame;
    let owner = tame && (game.mobs[index].extra.owner == Some(actor) || saved_owner);
    if tame {
        game.mobs[index].extra.tame = true;
        if owner { game.mobs[index].extra.owner = Some(actor); }
    }

    if food && owner && game.mobs[index].health < game.mobs[index].attrs.max_health {
        let nutrition = game.res.items().get(item.as_str())
            .and_then(|definition| definition.food.as_ref())
            .map_or(0.0, |properties| properties.0 as f32);
        if nutrition > 0.0 {
            if !game.creative { game.consume_held(); }
            game.mobs[index].heal(&game.loop_context, nutrition);
            return ControlFlow::Break(Ok(true));
        }
    }

    if food {
        if let Some(_outcome) = horizon_client::mobs::ai::feed_animal(
            &mut game.mobs[index], &mut game.world.random, Some(actor),
        ) {
            if !game.creative { game.consume_held(); }
            if game.mobs[index].extra.in_love > 0 {
                game.love_cause.insert(id, game.lan_acting);
                game.native_entity_enter_love_mode(id, kind.as_str(), game.lan_acting_uuid);
                game.mob_event_particles(id, 18);
            }
            return ControlFlow::Break(Ok(true));
        }
    }

    use horizon_client::{actions::Action, text::Text};
    use mod_api::SoundSource;
    let whistle = item == "naturalist:whistle"
        && !game.keys.held(Action::Sneak)
        && !game.cooldowns.is_on_cooldown(&item);
    if owner && item.is_empty() && !game.keys.held(Action::Sneak) {
        let sitting = game.mobs[index].extra.sitting;
        horizon_client::mobs::tame::restore_sitting(&mut game.mobs[index], !sitting);
        game.mobs[index].nav_stop();
        let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw {
            fields.insert("Sitting".into(), Tag::Byte(i8::from(!sitting)));
        }
        return ControlFlow::Break(Ok(true));
    }
    if owner && whistle {
        let following = game.persist.raw.get(&id).and_then(|tag| tag.get("FollowingOwner"))
            .and_then(horizon_world::datafix::dynamic::as_number)
            .is_none_or(|number| number.int_value() != 0);
        let at = game.mobs[index].center();
        horizon_client::mobs::tame::restore_sitting(&mut game.mobs[index], false);
        if following { game.mobs[index].ai.target = None; }
        let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw {
            fields.insert("FollowingOwner".into(), Tag::Byte(i8::from(!following)));
            fields.insert("Sitting".into(), Tag::Byte(0));
        }
        game.mobs[index].extra.ints[4] = i32::from(!following);
        game.add_cooldown(std::sync::Arc::from(item), 20);
        if let Some(level) = game.level(game.dim.id()) {
            level.play_sound(game, "naturalist:item.whistle", at, SoundSource::Neutral, 0.8, 1.0);
        }
        let name = game.mobs[index].custom_name.as_deref()
            .unwrap_or(if kind == "naturalist:lion" { "Lion" } else { "Tiger" }).to_owned();
        let mode = Text::translate(if following { "naturalist.whistle.wandering" } else { "naturalist.whistle.following" }, &[]);
        game.send_overlay_message(Text::translate("naturalist.whistle.message", &[Text::literal(name), mode]));
        return ControlFlow::Break(Ok(true));
    }
    ControlFlow::Continue(())
}
