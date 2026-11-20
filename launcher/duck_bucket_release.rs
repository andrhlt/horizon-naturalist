fn duck_bucket_release(game: &mut Game<'_>, stack: &ItemStack, claimed: &mut bool) {
    use horizon_client::mobs::finalize;
    let kind = "naturalist:duck";
    if *claimed || stack.item.as_ref() != "naturalist:duck_bucket" {
        return;
    }
    let Some(hit) = game.pick_unfiltered() else { return };
    let pos = hit.pos;
    if game.bucket_empty_cancelled(stack, pos) {
        *claimed = true;
        return;
    }
    let Some(id) = game.spawn_mob_at(kind, pos.into(), true) else { return };
    let bucket_data = stack.data().and_then(|data| data.other("minecraft:bucket_entity_data"));
    let mut at = [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + 0.5, f64::from(pos[2]) + 0.5];
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) {
        finalize::set_age(mob, -24000);
        horizon_client::entity_nbt::apply_item_components(mob, stack);
        mob.extra.from_bucket = true;
        mob.persistent = bucket_data
            .is_some_and(|data| horizon_client::entity_nbt::flag(data, "PersistenceRequired"));
        at = mob.center();
    }
    let variant = horizon_client_core::nbt_value::text(bucket_data.and_then(|data| data.get("Variant")));
    super::naturalist_deer_variants::put_variant(game, id, variant.as_deref().unwrap_or(kind));
    game.audio.play("minecraft:item.bucket.empty", Some(at), 1.0, 1.0);
    game.award_stat("used", &stack.item, 1);
    if !game.creative {
        *game.held_mut() = Some(ItemStack::new("minecraft:bucket", 1));
    }
    *claimed = true;
}
