fn snail_shell_color(
    game: &mut Game<'_>,
    pos: [i32; 3],
    state: BlockStateId,
    stack: &ItemStack,
) -> std::ops::ControlFlow<Result<BlockStateId>> {
    use horizon_client::mobs::render::DYE_NAMES;
    use std::ops::ControlFlow;
    if stack.item.as_ref() != super::naturalist_snail_shell::SHELL {
        return ControlFlow::Continue(());
    }
    const PROPERTIES: &str = "minecraft:block_state";
    let mut properties = match stack.data().and_then(|data| data.other(PROPERTIES)) {
        Some(Tag::Compound(values)) => values.clone(),
        _ => Default::default(),
    };
    if properties.contains_key(&"color".into()) {
        return ControlFlow::Continue(());
    }
    let color = stack
        .data()
        .and_then(|data| data.other("minecraft:custom_data"))
        .and_then(|data| horizon_client_core::nbt_value::number(data.get("Color")))
        .map(|color| DYE_NAMES.get(color as usize).copied().unwrap_or("white"))
        .unwrap_or("brown");
    properties.insert("color".into(), Tag::String(color.into()));
    let mut placed = stack.clone();
    placed
        .data_mut()
        .set_other(PROPERTIES, Some(Tag::Compound(properties)));
    ControlFlow::Break(game.apply_item_block_state(pos, state, &placed))
}
