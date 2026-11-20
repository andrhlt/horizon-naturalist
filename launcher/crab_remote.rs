fn crab_remote(game: &Game<'_>, index: usize, hand: Hand) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::nav::naturalist_crab as crab;
    use std::ops::ControlFlow;

    let Some(mob) = game.mobs.get(index).filter(|mob| mob.kind.id == crab::KIND && mob.alive()) else {
        return ControlFlow::Continue(());
    };
    let held = match hand {
        Hand::Main => game.stack_in(game.selected),
        Hand::Off => game.offhand.as_ref(),
    };
    if held.is_some_and(|stack| stack.item.as_ref() == "naturalist:capture_net") {
        return ControlFlow::Break(true);
    }
    ControlFlow::Continue(())
}
