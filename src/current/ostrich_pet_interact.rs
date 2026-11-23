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
    let actor = game.acting_player_uuid();
    let id = game.mobs[index].id;
    let baby = game.mobs[index].is_baby();
    let tame = game.mobs[index].extra.tame;
    let owner = tame && game.mobs[index].extra.owner == Some(actor);

    if !seeds {
        if kind == "naturalist:ostrich" && item == "minecraft:saddle" && tame && owner {
            use mod_api::{Host, HostTransaction, ItemPlace, ItemResource, EquipmentSlot};
            let saddle = ItemPlace::Equipment { entity: id, slot: EquipmentSlot::Saddle };
            if saddle.stack(game).is_none() {
                if let Some(stack) = game.held().cloned() {
                    let moved = game.transaction(|host, tx| {
                        Ok::<u32, ()>(saddle.insert(host, tx, &ItemResource::of(&stack), 1))
                    }).unwrap_or(0);
                    if moved == 1 {
                        game.consume_held();
                        return ControlFlow::Break(Ok(true));
                    }
                }
            }
        }
        return ControlFlow::Continue(());
    }

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

    // ServerHost exposes the same mount path used by /ride.  Keep the
    // interaction owner-gated, matching Ostrich.mobInteract.
    if kind == "naturalist:ostrich" && owner && game.held().is_none() {
        use mod_api::ServerHost;
        if let Some(rider) = game.entity(game.native_acting_entity()) {
            if game.entity_start_riding(rider, game.entity(id).expect("target entity"), false) {
                return ControlFlow::Break(Ok(true));
            }
        }
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
