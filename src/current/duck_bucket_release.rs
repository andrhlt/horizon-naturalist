fn duck_bucket_release(game: &mut Game<'_>, call: mod_api::__private::UseItemCall<'_>, original: impl FnOnce(&mut Game<'_>, mod_api::__private::UseItemCall<'_>) -> ()) {
    use horizon_client::items::ItemStack;
    use horizon_core::nbt::Tag;
    use mod_api::{Host, InteractionResult, SoundSource};
    let Some(stack) = game.held().cloned() else { original(game, call); return };
    if stack.item.as_ref() != "naturalist:duck_bucket" || game.remote.is_some() { original(game, call); return }
    let Some(hit) = game.pick_unfiltered() else { original(game, call); return };
    let pos = hit.pos;
    if game.bucket_empty_cancelled(&stack, pos) { *call.result = Some(InteractionResult::Fail); return }
    let Some(id) = game.spawn_mob_at("naturalist:duck", pos.into(), true) else { original(game, call); return };
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) {
        horizon_client::mobs::finalize::set_age(mob, -24000, None);
        horizon_client::entity_nbt::apply_item_components(mob, &stack);
        mob.extra.from_bucket = true;
    }
    let variant = stack.data().and_then(|d| d.other("minecraft:bucket_entity_data")).and_then(|d| d.get("Variant")).cloned().unwrap_or_else(|| Tag::String("naturalist:duck".into()));
    let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
    if let Tag::Compound(fields) = raw { fields.insert("Variant".into(), variant); }
    if let Some(level) = game.level(game.dim.id()) { level.play_sound(game, "minecraft:item.bucket.empty", [pos[0] as f64 + 0.5, pos[1] as f64 + 0.5, pos[2] as f64 + 0.5], SoundSource::Neutral, 1.0, 1.0); }
    game.award_stat("used", &stack.item, 1);
    if !game.creative { *game.held_mut() = Some(ItemStack::new("minecraft:bucket", 1)); }
    *call.result = Some(InteractionResult::SuccessServer);
}
