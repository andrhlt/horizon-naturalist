fn anglerfish_bucket_pickup(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::{entity_nbt, items::ItemStack};
    use horizon_core::nbt::Tag;
    use mod_api::Host;
    use std::ops::ControlFlow;

    let Some(held) = game
        .held()
        .filter(|stack| stack.item.as_ref() == "minecraft:water_bucket")
        .cloned()
    else {
        return ControlFlow::Continue(());
    };
    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let mob = &game.mobs[index];
    let (bucket, default_variant) = match mob.kind.id.as_str() {
        "naturalist:anglerfish" => ("naturalist:anglerfish_bucket", "naturalist:red"),
        "naturalist:blobfish" => ("naturalist:blobfish_bucket", "naturalist:blobfish"),
        "naturalist:giant_isopod" => ("naturalist:giant_isopod_bucket", "naturalist:brown"),
        "naturalist:jellyfish" => ("naturalist:jellyfish_bucket", "naturalist:white"),
        "naturalist:ray" => ("naturalist:ray_bucket", "naturalist:eagle_ray"),
        "naturalist:starfish" => ("naturalist:starfish_bucket", "naturalist:orange"),
        _ => return ControlFlow::Continue(()),
    };
    if !mob.alive() {
        return ControlFlow::Continue(());
    }
    let id = mob.id;
    let kind = mob.kind.id.clone();
    let variant = game
        .persist
        .raw
        .get(&id)
        .and_then(|raw| raw.get("Variant"))
        .cloned()
        .unwrap_or_else(|| Tag::String(default_variant.into()));
    let mut data = entity_nbt::bucket_components(mob);
    let mut entity_data = entity_nbt::bucket_data(mob);
    if let Tag::Compound(fields) = &mut entity_data {
        fields.insert("Variant".into(), variant.clone());
    }
    data.set_other("minecraft:bucket_entity_data", Some(entity_data));
    data.set_other(
        "minecraft:custom_data",
        Some(Tag::compound([("Variant", variant)])),
    );
    let mut filled = ItemStack::new(bucket, 1);
    filled.extra = Some(Box::new(data));
    let target_entity = game.mob_entity(mob);
    if let Some(entity) = game.entity(id) {
        entity.play_sound(game, "minecraft:item.bucket.fill_fish", 1.0, 1.0);
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
        game.native_player_bucket_entity(player, &name, id as i32, &kind);
    }
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
