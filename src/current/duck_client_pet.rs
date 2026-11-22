fn duck_client_pet(game: &mut Game<'_>, index: usize, hand: Hand) -> std::ops::ControlFlow<bool> {
    use horizon_client::actions::Action;
    use std::ops::ControlFlow;
    if hand != Hand::Main || !game.mobs.get(index).is_some_and(|m| m.kind.id == "naturalist:duck" && m.alive()) { return ControlFlow::Continue(()) }
    let held = game.stack_in(game.selected);
    let food = held.is_some_and(|s| game.items().item_tags.get("naturalist:duck_food_items").is_some_and(|tag| tag.contains(s.item.as_ref())));
    let donor_food = held.is_some_and(|s|game.mobs[index].kind.food.iter().any(|id|id==s.item.as_ref()));
    let dye_or_shears = held.is_some_and(|s| {
        let dye = game.items().get(&s.item).and_then(|info|info.dye.as_ref()).is_some()
            && !s.data().is_some_and(|d|d.other("!minecraft:dye").is_some());
        dye || game.items().item_tags.get("naturalist:shears").is_some_and(|tag|tag.contains(s.item.as_ref()))
    });
    if food || donor_food || dye_or_shears || held.is_none() || held.is_some_and(|s| s.item.as_ref() == "naturalist:whistle") && !game.keys.held(Action::Sneak) { return ControlFlow::Break(true) }
    ControlFlow::Continue(())
}
