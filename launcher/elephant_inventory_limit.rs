fn elephant_inventory_limit(menu: &Menu, slot: usize) -> std::ops::ControlFlow<u32> {
    if matches!(menu.kind, Kind::Mod(kind) if matches!(kind.id, "naturalist:elephant" | "naturalist:elephant_unchested"))
        && matches!(slot, 25 | 26)
    {
        std::ops::ControlFlow::Break(1)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
