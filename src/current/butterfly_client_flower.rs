fn butterfly_client_flower(game: &mut Game<'_>, index: usize, hand: Hand) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if !game.mobs.get(index).is_some_and(|m| m.kind.id == "naturalist:butterfly" && m.alive()) { return ControlFlow::Continue(()) }
    let held = match hand { Hand::Main => game.stack_in(game.selected), Hand::Off => game.offhand.as_ref() };
    if held.is_some_and(|s| game.items().item_tags.get("minecraft:flowers").is_some_and(|tag| tag.contains(s.item.as_ref()))) { return ControlFlow::Break(true) }
    ControlFlow::Continue(())
}
