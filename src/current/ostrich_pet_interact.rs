fn ostrich_pet_interact(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_core::nbt::Tag;
    use std::ops::ControlFlow;

    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let kind = game.mobs[index].kind.id.as_str();
    if !matches!(kind, "naturalist:ostrich" | "naturalist:turkey")
        || !game.mobs[index].alive()
    {
        return ControlFlow::Continue(());
    }
    let item = game.held().map(|stack| stack.item.to_string()).unwrap_or_default();
    let seeds = matches!(
        item.as_str(),
        "minecraft:wheat_seeds"
            | "minecraft:beetroot_seeds"
            | "minecraft:melon_seeds"
            | "minecraft:pumpkin_seeds"
            | "minecraft:torchflower_seeds"
            | "minecraft:pitcher_pod"
    );
    if !seeds {
        return ControlFlow::Continue(());
    }

    let actor = game.acting_player_uuid();
    let id = game.mobs[index].id;
    let baby = game.mobs[index].is_baby();
    let tame = game.mobs[index].extra.tame;
    let owner = tame && game.mobs[index].extra.owner == Some(actor);

    // OstrichProbe: an untamed juvenile accepts its first seed as a tame,
    // ordered companion.  The Java implementation is deterministic here.
    if kind == "naturalist:ostrich" && baby && !tame {
        game.consume_held();
        game.mobs[index].extra.tame = true;
        game.mobs[index].extra.owner = Some(actor);
        horizon_client::mobs::tame::restore_sitting(&mut game.mobs[index], true);
        let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw {
            fields.insert("Tame".into(), Tag::Byte(1));
            fields.insert("Owner".into(), Tag::IntArray(actor.to_vec()));
            fields.insert("Sitting".into(), Tag::Byte(1));
        }
        return ControlFlow::Break(Ok(true));
    }

    if kind == "naturalist:ostrich" && owner && baby {
        game.consume_held();
        horizon_client::mobs::ai::age_up(&mut game.mobs[index], 120, true);
        return ControlFlow::Break(Ok(true));
    }

    if owner && game.mobs[index].health < game.mobs[index].attrs.max_health {
        game.consume_held();
        game.mobs[index].heal(&game.loop_context, 2.0);
        return ControlFlow::Break(Ok(true));
    }

    // The LandProbe seed set is broader than the native donor-food list;
    // retain the exact Java outcomes for Turkey and adult Ostrich breeding.
    if !baby {
        game.consume_held();
        game.mobs[index].extra.in_love = 600;
        game.mobs[index].extra.love_cause = Some(actor);
        game.mobs[index].extra.love_player = true;
        return ControlFlow::Break(Ok(true));
    }

    ControlFlow::Continue(())
}
