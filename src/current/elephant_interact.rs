fn elephant_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_core::nbt::Tag;
    use std::ops::ControlFlow;

    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let kind = game.mobs[index].kind.id.as_ref();
    if !matches!(kind, "naturalist:elephant" | "naturalist:mammoth") || !game.mobs[index].alive() {
        return ControlFlow::Continue(());
    }
    let id = game.mobs[index].id;
    let actor = game.acting_player_uuid();
    let item = game.held().map(|stack| stack.item.to_string()).unwrap_or_default();
    let baby = game.mobs[index].is_baby();
    let raw_pet = game.persist.raw.get(&id);
    let saved_tame = raw_pet.and_then(|tag| tag.get("Tame"))
        .is_some_and(|value| matches!(value, Tag::Byte(flag) if *flag != 0));
    let saved_owner = raw_pet.and_then(|tag| tag.get("Owner"))
        .and_then(|value| match value { Tag::IntArray(words) if words.as_slice() == actor => Some(()), _ => None })
        .is_some();
    let tame = game.mobs[index].extra.tame || saved_tame;
    let owner = tame && (game.mobs[index].extra.owner == Some(actor) || saved_owner);

    // Elephant.mobInteract: five cakes tame only a baby. Creative players
    // still advance the counter but do not consume the cake.
    if baby && !tame && item == "minecraft:cake" {
        if !game.creative {
            game.consume_held();
        }
        let raw = game
            .persist
            .raw
            .entry(id)
            .or_insert_with(|| Tag::Compound(Default::default()));
        let progress = if let Tag::Compound(fields) = raw {
            let previous = fields
                .get(&"TamingFood".into())
                .and_then(|value| match value {
                    Tag::Int(value) => Some(*value),
                    _ => None,
                })
                .unwrap_or(0);
            let next = previous.saturating_add(1).min(5);
            fields.insert("TamingFood".into(), Tag::Int(next));
            if next == 5 {
                fields.insert("Tame".into(), Tag::Byte(1));
                fields.insert("Owner".into(), Tag::IntArray(actor.to_vec()));
            }
            next
        } else {
            0
        };
        if progress == 5 {
            game.mobs[index].extra.tame = true;
            game.mobs[index].extra.owner = Some(actor);
            let view = game.mob_entity(&game.mobs[index]);
            game.trigger_entity("minecraft:tame_animal", "entity", &view);
        }
        game.mob_event_particles(id, if progress == 5 { 7 } else { 6 });
        return ControlFlow::Break(Ok(true));
    }

    // Java delegates this path to the superclass for non-owners.
    if !owner || baby {
        return ControlFlow::Continue(());
    }

    if item == "minecraft:chest" && !game.mobs[index].extra.chested {
        game.mobs[index].extra.chested = true;
        game.mobs[index].extra.items.resize(15, None);
        let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw { fields.insert("ChestedHorse".into(), Tag::Byte(1)); }
        if !game.creative { game.consume_held(); }
        return ControlFlow::Break(Ok(true));
    }

    if item == "minecraft:saddle" && !game.mobs[index].extra.saddled() {
        game.mobs[index].extra.set_saddled(true);
        let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw { fields.insert("Saddled".into(), Tag::Byte(1)); }
        if !game.creative { game.consume_held(); }
        return ControlFlow::Break(Ok(true));
    }

    if item == "minecraft:shears" && (game.mobs[index].extra.chested || game.mobs[index].extra.saddled()) {
        let at = game.mobs[index].position.components();
        if game.mobs[index].extra.chested {
            game.mobs[index].extra.chested = false;
            let contents = std::mem::take(&mut game.mobs[index].extra.items);
            for stack in contents.into_iter().flatten() {
                game.spawn_item(stack, at, [0.0, 0.2, 0.0], 10);
            }
            game.spawn_item(horizon_client::items::ItemStack::new("minecraft:chest", 1), at, [0.0, 0.2, 0.0], 10);
            let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
            if let Tag::Compound(fields) = raw { fields.insert("ChestedHorse".into(), Tag::Byte(0)); }
        } else {
            game.mobs[index].extra.set_saddled(false);
            game.spawn_item(horizon_client::items::ItemStack::new("minecraft:saddle", 1), at, [0.0, 0.2, 0.0], 10);
            let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
            if let Tag::Compound(fields) = raw { fields.insert("Saddled".into(), Tag::Byte(0)); }
        }
        game.damage_held_tool();
        return ControlFlow::Break(Ok(true));
    }

    if item.is_empty() && game.mobs[index].extra.saddled() {
        if game.native_may_board(id) {
            game.start_riding(id);
            return ControlFlow::Break(Ok(true));
        }
    }

    if item != "minecraft:melon_slice" {
        return ControlFlow::Continue(());
    }
    ControlFlow::Continue(())
}
