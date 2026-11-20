fn elephant_inventory_quick_move(
    menu: &mut Menu,
    env: &Env<'_>,
    index: usize,
) -> std::ops::ControlFlow<Option<ItemStack>> {
    use std::ops::ControlFlow;

    const CHEST: usize = 25;
    const SADDLE: usize = 25;
    const BANNER: usize = 26;
    const CONTAINER: usize = 27;
    if !matches!(menu.kind, Kind::Mod(kind) if matches!(kind.id, "naturalist:elephant" | "naturalist:elephant_unchested")) {
        return ControlFlow::Continue(());
    }
    let chested = matches!(menu.kind, Kind::Mod(kind) if kind.id == "naturalist:elephant");
    if !chested && index < CHEST {
        return ControlFlow::Break(None);
    }
    let Some(mut stack) = menu.slot(index).cloned() else { return ControlFlow::Break(None) };
    let original = stack.clone();
    let total = menu.slots.len();
    let moved = if index < CONTAINER {
        menu.move_to(env, &mut stack, CONTAINER, total, true)
    } else if menu.slot(SADDLE).is_none() && menu.may_place(env, SADDLE, &stack) {
        menu.move_to(env, &mut stack, SADDLE, SADDLE + 1, false)
    } else if menu.slot(BANNER).is_none() && menu.may_place(env, BANNER, &stack) {
        menu.move_to(env, &mut stack, BANNER, BANNER + 1, false)
    } else {
        chested && menu.move_to(env, &mut stack, 0, CHEST, false)
    };
    if moved {
        menu.put(env, index, (stack.count > 0).then_some(stack));
    }
    ControlFlow::Break(moved.then_some(original))
}
