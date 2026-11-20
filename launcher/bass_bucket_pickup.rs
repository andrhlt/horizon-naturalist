fn bass_bucket_pickup(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_world::nbt::Tag;
    use std::ops::ControlFlow;

    let Some(held) = game.held()
        .filter(|stack| matches!(stack.item.as_ref(), "minecraft:water_bucket" | "minecraft:bucket")).cloned()
    else {
        return ControlFlow::Continue(());
    };
    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let mob = &game.mobs[index];
    if held.item.as_ref() == "minecraft:bucket" {
        return if mob.kind.id == "naturalist:giant_isopod" {
            ControlFlow::Break(Ok(false))
        } else {
            ControlFlow::Continue(())
        };
    }
    let (kind, item) = match mob.kind.id.as_str() {
        "naturalist:bass" => ("naturalist:bass", "naturalist:bass_bucket"),
        "naturalist:catfish" => ("naturalist:catfish", "naturalist:catfish_bucket"),
        "naturalist:piranha" => ("naturalist:piranha", "naturalist:piranha_bucket"),
        "naturalist:blobfish" => ("naturalist:blobfish", "naturalist:blobfish_bucket"),
        "naturalist:jellyfish" => ("naturalist:jellyfish", "naturalist:jellyfish_bucket"),
        "naturalist:starfish" => ("naturalist:starfish", "naturalist:starfish_bucket"),
        "naturalist:giant_isopod" => ("naturalist:giant_isopod", "naturalist:giant_isopod_bucket"),
        "naturalist:anglerfish" => ("naturalist:anglerfish", "naturalist:anglerfish_bucket"),
        "naturalist:ray" => ("naturalist:ray", "naturalist:ray_bucket"),
        _ => return ControlFlow::Continue(()),
    };
    if !mob.alive() {
        return ControlFlow::Continue(());
    }
    let id = mob.id;
    let mut data = horizon_client::entity_nbt::bucket_components(mob);
    if matches!(kind, "naturalist:bass" | "naturalist:blobfish" | "naturalist:jellyfish" | "naturalist:ray" | "naturalist:anglerfish" | "naturalist:starfish" | "naturalist:giant_isopod") {
    let variant = game.persist.raw.get(&id).and_then(|data| data.get("Variant")).cloned()
        .unwrap_or_else(|| Tag::String(match kind { "naturalist:jellyfish" => "naturalist:white", "naturalist:ray" => "naturalist:eagle_ray", "naturalist:anglerfish" => "naturalist:red", "naturalist:starfish" => "naturalist:orange", "naturalist:giant_isopod" => "naturalist:brown", other => other }.into()));
    let mut entity_data = horizon_client::entity_nbt::bucket_data(mob);
    if let Tag::Compound(fields) = &mut entity_data {
        fields.insert("Variant".into(), variant.clone());
    }
    data.set_other("minecraft:bucket_entity_data", Some(entity_data));
    if matches!(kind, "naturalist:bass" | "naturalist:jellyfish" | "naturalist:ray" | "naturalist:anglerfish" | "naturalist:starfish" | "naturalist:giant_isopod") {
        data.set_other("minecraft:custom_data", Some(Tag::compound([("Variant", variant)])));
    }
    }
    let mut filled = ItemStack::new(item, 1);
    filled.extra = Some(Box::new(data));
    let at = mob.center();
    let entity = game.mob_entity(mob);
    game.audio.play("minecraft:item.bucket.fill_fish", Some(at), 1.0, 1.0);
    game.trigger_item("minecraft:filled_bucket", &filled);
    game.trigger_item_entity("minecraft:player_interacted_with_entity", &held, &entity);
    game.break_leash(id, true, false);
    game.mobs[index].removed = true;
    game.filled_result(filled);
    if let (Some(player), Some(name)) = (game.lan_acting_uuid, game.lan_acting_name.clone()) {
        game.native_player_bucket_entity(player, &name, id as i32, kind);
    }
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
