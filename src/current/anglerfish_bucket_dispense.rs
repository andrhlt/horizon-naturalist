fn anglerfish_bucket_dispense(
    game: &mut Game<'_>,
    target: Pos,
    stack: &ItemStack,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_world::level::BucketContent;
    use std::ops::ControlFlow;

    if stack.item.as_ref() != "naturalist:anglerfish_bucket" {
        return ControlFlow::Continue(());
    }
    let target_pos = Game::pos_of(target);
    let content = BucketContent::MobWater;
    let emptied = game.with_level(|block, level, ticks| {
        block.bucket_empty(
            level,
            ticks,
            target_pos,
            None,
            content,
            "minecraft:item.bucket.empty_fish",
            false,
            false,
        )
    });
    if let Err(error) = game.process_level_events() {
        return ControlFlow::Break(Err(error));
    }
    if emptied.is_none() {
        return ControlFlow::Break(Ok(false));
    }
    if game.dimension_type().water_evaporates {
        game.evaporation_smoke([target.0, target.1, target.2]);
    }
    if let Some(id) = game.spawn_bucket_mob("naturalist:anglerfish", target) {
        if let Err(error) = crate::game::naturalist_anglerfish_bucket::restore(game, id, stack) {
            return ControlFlow::Break(Err(error));
        }
    }
    game.post_block_game_event("minecraft:entity_place", [target.0, target.1, target.2], None, None);
    ControlFlow::Break(Ok(true))
}
