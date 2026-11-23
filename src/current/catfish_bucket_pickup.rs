fn catfish_bucket_pickup(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::{entity_nbt, items::ItemStack};
    use mod_api::Host;
    use std::ops::ControlFlow;
    let Some(held) = game.held().filter(|stack| stack.item.as_ref() == "minecraft:water_bucket").cloned() else {
        return ControlFlow::Continue(());
    };
    let Some(index) = game.mob_interaction_target(target) else { return ControlFlow::Continue(()) };
    let mob = &game.mobs[index];
    if mob.kind.id != "naturalist:catfish" || !mob.alive() { return ControlFlow::Continue(()) }
    let id = mob.id;
    let mut data = entity_nbt::bucket_components(mob);
    data.set_other("minecraft:bucket_entity_data", Some(entity_nbt::bucket_data(mob)));
    let mut filled = ItemStack::new("naturalist:catfish_bucket", 1);
    filled.extra = Some(Box::new(data));
    let target_entity = game.mob_entity(mob);
    if let Some(entity) = game.entity(id) { entity.play_sound(game, "minecraft:item.bucket.fill_fish", 1.0, 1.0); }
    game.trigger_item("minecraft:filled_bucket", &filled);
    game.trigger_item_entity("minecraft:player_interacted_with_entity", &held, &target_entity);
    game.break_leash(id, true, false);
    game.mobs[index].removed = true;
    game.filled_result(filled);
    if let (Some(player), Some(name)) = (game.lan_acting_uuid, game.lan_acting_name.clone()) {
        game.native_player_bucket_entity(player, &name, id as i32, "naturalist:catfish");
    }
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
