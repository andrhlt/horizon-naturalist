fn tortoise_egg_pick(
    game: &Game<'_>,
    block: &str,
    state: BlockStateId,
    stack: &mut Option<ItemStack>,
) {
    if stack.is_some() || block != "naturalist:tortoise_egg" {
        return;
    }
    let Some(variant) = game.pack.assets.registry.state(state)
        .and_then(|row| row.properties.get("variant"))
        .and_then(|value| value.parse::<i32>().ok())
    else {
        return;
    };
    use horizon_world::nbt::Tag;
    let mut egg = ItemStack::new(block, 1);
    egg.data_mut().set_other("minecraft:custom_data", Some(Tag::compound([
        ("Variant", Tag::Int(variant)),
    ])));
    *stack = Some(egg);
}
