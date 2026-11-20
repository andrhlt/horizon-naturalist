fn snail_shell_mining(
    game: &mut Game<'_>,
    pos: [i32; 3],
    state: BlockStateId,
    held: Option<&ItemStack>,
    entity: &Option<horizon_world::nbt::Tag>,
) -> std::ops::ControlFlow<Vec<ItemStack>> {
    use std::ops::ControlFlow;
    naturalist_glow_goop::mined(game, pos, state);
    let flower = entity.as_ref().and_then(|nbt| nbt.get("Flower"));
    let Some(mut drops) = naturalist_snail_shell::loot(game.pack.assets.registry, state, flower)
    else {
        return ControlFlow::Continue(());
    };
    let player = game.native_acting_id();
    game.native_block_drops(pos, state, Some(player), held, None, &mut drops);
    ControlFlow::Break(drops)
}
