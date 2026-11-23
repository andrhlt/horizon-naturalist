fn duck_bucket_pickup(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::{entity_nbt, items::ItemStack};
    use horizon_core::nbt::Tag;
    use mod_api::Host;
    use std::ops::ControlFlow;

    let Some(held) = game
        .held()
        .filter(|stack| stack.item.as_ref() == "minecraft:bucket")
        .cloned()
    else {
        return ControlFlow::Continue(());
    };
    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let actor = game.acting_player_uuid();
    let mob = &game.mobs[index];
    if mob.kind.id != "naturalist:duck" || !mob.alive() {
        return ControlFlow::Continue(());
    }
    if !mob.is_baby() {
        // A matching remote client claims Duck bucket clicks before local age
        // metadata can arrive. Let the authoritative server decide whether
        // this is a baby capture or Java's ordinary item-use fallback.
        let capable = game
            .lan_acting_uuid
            .and_then(|uuid| game.player(uuid))
            .and_then(|player| game.player_announced_channels(player))
            .is_some_and(|channels| channels.iter().any(|channel| channel == "naturalist:duck_bucket_atomic_v1"));
        if capable && let Some(crate::game::GuestUse::Entity(hand, _)) = game.guest_use {
            let previous = game.guest_use;
            game.guest_use = Some(crate::game::GuestUse::Item(hand));
            let result = game.use_item();
            game.guest_use = previous;
            return match result {
                Ok(value) => ControlFlow::Break(Ok(value.is_some())),
                Err(error) => ControlFlow::Break(Err(error)),
            };
        }
        return ControlFlow::Continue(());
    }
    if mob.extra.tame && mob.extra.owner != Some(actor) {
        // On a remote client's entity packet, this mod's client claims the
        // interaction, so execute the normal Item.use fallback on the server.
        // Singleplayer already performs its own fallthrough.
        let capable = game.lan_acting_uuid
            .and_then(|uuid|game.player(uuid))
            .and_then(|player|game.player_announced_channels(player))
            .is_some_and(|channels|channels.iter().any(|channel|channel=="naturalist:duck_bucket_atomic_v1"));
        if capable && let Some(crate::game::GuestUse::Entity(hand, _)) = game.guest_use {
            let previous = game.guest_use;
            game.guest_use = Some(crate::game::GuestUse::Item(hand));
            let result = game.use_item();
            game.guest_use = previous;
            return match result {
                Ok(value) => ControlFlow::Break(Ok(value.is_some())),
                Err(error) => ControlFlow::Break(Err(error)),
            };
        }
        return ControlFlow::Continue(());
    }
    let id = mob.id;
    let variant = game
        .persist
        .raw
        .get(&id)
        .and_then(|raw| raw.get("Variant"))
        .cloned()
        .unwrap_or_else(|| Tag::String("naturalist:duck".into()));
    let mut data = entity_nbt::bucket_components(mob);
    let mut entity_data = entity_nbt::bucket_data(mob);
    if let Tag::Compound(fields) = &mut entity_data {
        fields.insert("Variant".into(), variant.clone());
        fields.insert("Age".into(), Tag::Int(mob.extra.age));
    }
    data.set_other("minecraft:bucket_entity_data", Some(entity_data));
    data.set_other(
        "minecraft:custom_data",
        Some(Tag::compound([("Variant", variant)])),
    );
    let mut filled = ItemStack::new("naturalist:duck_bucket", 1);
    filled.extra = Some(Box::new(data));
    let target_entity = game.mob_entity(mob);
    if let Some(entity) = game.entity(id) {
        entity.play_sound(game, "minecraft:item.bucket.fill", 1.0, 1.0);
    }
    game.trigger_item("minecraft:filled_bucket", &filled);
    game.trigger_item_entity(
        "minecraft:player_interacted_with_entity",
        &held,
        &target_entity,
    );
    game.break_leash(id, true, false);
    game.mobs[index].removed = true;
    game.filled_result(filled);
    if let (Some(player), Some(name)) = (game.lan_acting_uuid, game.lan_acting_name.clone()) {
        game.native_player_bucket_entity(player, &name, id as i32, "naturalist:duck");
    }
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
