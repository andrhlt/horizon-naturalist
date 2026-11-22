fn rat_client_pet(game: &mut Game<'_>, index: usize, hand: Hand) -> std::ops::ControlFlow<bool> {
    use horizon_client::actions::Action;
    use std::ops::ControlFlow;
    if hand != Hand::Main || !game.mobs.get(index).is_some_and(|m| m.kind.id == "naturalist:rat" && m.alive()) { return ControlFlow::Continue(()) }
    let held = game.stack_in(game.selected);
    let food = held.is_some_and(|s| s.item.as_ref() == "minecraft:bread");
    let capture_net = held.is_some_and(|s| s.item.as_ref() == "naturalist:capture_net");
    if food || (!capture_net && !game.keys.held(Action::Sneak)) { return ControlFlow::Break(true) }
    ControlFlow::Continue(())
}
