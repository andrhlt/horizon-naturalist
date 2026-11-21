fn duck_client_bucket(game: &mut Game<'_>, index: usize, hand: Hand) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    let held=match hand { Hand::Main => game.stack_in(game.selected), Hand::Off => game.offhand.as_ref() };
    if held.is_some_and(|s|s.item.as_ref()=="minecraft:bucket") && game.mobs.get(index).is_some_and(|m|m.kind.id=="naturalist:duck" && m.alive() && m.is_baby()) {
        return ControlFlow::Break(true);
    }
    ControlFlow::Continue(())
}
