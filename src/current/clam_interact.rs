fn clam_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_core::nbt::Tag;
    use std::ops::ControlFlow;

    let Some(index) = game.mob_interaction_target(target) else { return ControlFlow::Continue(()) };
    if game.mobs[index].kind.id != "naturalist:clam" || !game.mobs[index].alive() {
        return ControlFlow::Continue(());
    }
    let id = game.mobs[index].id;
    let held = game.held().cloned();
    if let Some(stack) = game.mobs[index].extra.main_hand_stack().cloned() {
        game.mobs[index].extra.set_main_hand_stack(None);
        let at = game.player.as_ref().map(|player| player.position).unwrap_or(game.mobs[index].position);
        game.spawn_at_location(at, 0.0, stack);
        let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw {
            fields.insert("NaturalistClamOpen".into(), Tag::Byte(0));
            fields.insert("NaturalistClamTimer".into(), Tag::Int(300));
            fields.insert("NaturalistClamCooldown".into(), Tag::Int(60));
            fields.insert("NaturalistClamLaunchDelay".into(), Tag::Int(0));
        }
        return ControlFlow::Break(Ok(true));
    }
    if game.creative && held.as_ref().is_some_and(|stack| stack.item.as_ref() == "minecraft:debug_stick") {
        let open = game.persist.raw.get(&id).and_then(|raw| raw.get("NaturalistClamOpen"))
            .and_then(horizon_world::datafix::dynamic::as_number).is_none_or(|value| value.int_value() == 0);
        let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw {
            fields.insert("NaturalistClamOpen".into(), Tag::Byte(i8::from(open)));
            fields.insert("NaturalistClamTimer".into(), Tag::Int(200));
            fields.insert("NaturalistClamCooldown".into(), Tag::Int(200));
            fields.insert("NaturalistClamLaunchDelay".into(), Tag::Int(0));
        }
        return ControlFlow::Break(Ok(true));
    }
    let Some(mut treasure) = held.filter(|stack| stack.count > 0) else { return ControlFlow::Continue(()) };
    treasure.count = 1;
    game.mobs[index].extra.set_main_hand_stack(Some(treasure));
    game.consume_held();
    let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
    if let Tag::Compound(fields) = raw {
        fields.insert("NaturalistClamOpen".into(), Tag::Byte(1));
        fields.insert("NaturalistClamTimer".into(), Tag::Int(200));
        fields.insert("NaturalistClamCooldown".into(), Tag::Int(60));
        fields.insert("NaturalistClamLaunchDelay".into(), Tag::Int(0));
    }
    ControlFlow::Break(Ok(true))
}
