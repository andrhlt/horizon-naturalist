fn elephant_inventory_accepts(
    menu: &Menu,
    env: &Env<'_>,
    slot: usize,
    stack: &ItemStack,
) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;

    if !matches!(menu.kind, Kind::Mod(kind) if matches!(kind.id, "naturalist:elephant" | "naturalist:elephant_unchested")) {
        return ControlFlow::Continue(());
    }
    match slot {
        0..25 if matches!(menu.kind, Kind::Mod(kind) if kind.id == "naturalist:elephant_unchested") =>
            ControlFlow::Break(false),
        25 => ControlFlow::Break(stack.item.as_ref() == "minecraft:saddle"),
        26 => ControlFlow::Break(env.items.item_tags.get("minecraft:banners")
            .is_some_and(|items| items.contains(stack.item.as_ref()))),
        _ => ControlFlow::Continue(()),
    }
}
