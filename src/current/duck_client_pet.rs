fn duck_client_pet(game: &mut Game<'_>, index: usize, hand: Hand) -> std::ops::ControlFlow<bool> {
    use horizon_client::actions::Action;
    use std::ops::ControlFlow;
    if hand != Hand::Main || !game.mobs.get(index).is_some_and(|m| m.kind.id == "naturalist:duck" && m.alive()) { return ControlFlow::Continue(()) }
    let held = game.stack_in(game.selected);
    if held.is_none() || held.is_some_and(|s| s.item.as_ref() == "naturalist:whistle") && !game.keys.held(Action::Sneak) { return ControlFlow::Break(true) }
    ControlFlow::Continue(())
}
