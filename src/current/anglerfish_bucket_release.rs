fn anglerfish_bucket_release(
    game: &mut Game<'_>,
    call: mod_api::__private::UseItemCall<'_>,
    original: impl FnOnce(&mut Game<'_>, mod_api::__private::UseItemCall<'_>) -> (),
) {
    use horizon_client::actions::Action as Mapping;
    use horizon_client::items::ItemStack;
    use horizon_core::position::BlockPos;
    use horizon_world::level::BucketContent;
    use mod_api::InteractionResult;

    let Some(stack) = game.held().cloned() else {
        original(game, call);
        return;
    };
    if stack.item.as_ref() != "naturalist:anglerfish_bucket" || game.remote.is_some() {
        original(game, call);
        return;
    }
    let Some(hit) = game.pick() else {
        original(game, call);
        return;
    };
    let content = BucketContent::MobWater;
    let sound = "minecraft:item.bucket.empty_fish";
    if game.bucket_use_refused(hit.pos) {
        *call.result = Some(InteractionResult::Fail);
        return;
    }
    let target = game.bucket_target(content, &hit);
    let mut canceled = false;
    game.native_fill_bucket(&stack, target, &mut canceled);
    let beside_too = hit.beside() != target;
    if !canceled && beside_too {
        game.native_fill_bucket(&stack, hit.beside(), &mut canceled);
    }
    if canceled
        || game.bucket_empty_cancelled(&stack, target)
        || beside_too && game.bucket_empty_cancelled(&stack, hit.beside())
    {
        *call.result = Some(InteractionResult::Fail);
        return;
    }
    let pos = BlockPos::new(target[0], target[1], target[2]);
    let [x, y, z] = hit.beside();
    let beside = Some(BlockPos::new(x, y, z));
    let sneaking = game.keys.held(Mapping::Sneak);
    let creative = game.creative;
    let emptied = game.with_level(|block, level, ticks| {
        block.bucket_empty(
            level, ticks, pos, beside, content, sound, sneaking, creative,
        )
    });
    if game.process_level_events().is_err() {
        *call.result = Some(InteractionResult::Fail);
        return;
    }
    let Some(emptied) = emptied else {
        *call.result = Some(InteractionResult::Fail);
        return;
    };
    if game.dimension_type().water_evaporates {
        game.evaporation_smoke([emptied.x, emptied.y, emptied.z]);
    }
    game.keep_guest_bucket_sounds(sound);
    if let Some(id) = game.spawn_bucket_mob("naturalist:anglerfish", (target[0], target[1], target[2])) {
        if crate::game::naturalist_anglerfish_bucket::restore(game, id, &stack).is_err() {
            *call.result = Some(InteractionResult::Fail);
            return;
        }
    }
    game.post_block_game_event("minecraft:entity_place", target, game.sculk_actor(), None);
    game.trigger_block("minecraft:placed_block", target, &stack);
    game.bucket_emptied(&stack.item);
    if !game.creative {
        *game.held_mut() = Some(ItemStack::new("minecraft:bucket", 1));
    }
    *call.result = Some(InteractionResult::SuccessServer);
}
