fn hedgehog_primary_enchant(
    enchants: &Enchantments,
    items: &Items,
    entry: &Enchantment,
    item: &str,
) -> std::ops::ControlFlow<bool> {
    if item == "naturalist:hedgehog" {
        std::ops::ControlFlow::Break(enchants.can_enchant(items, &entry.id, item))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
