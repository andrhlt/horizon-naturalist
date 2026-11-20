fn capture_net_remote(game: &Game<'_>, index: usize, hand: Hand) -> std::ops::ControlFlow<bool> {
    use super::naturalist_capture_net_swing::{can_catch_vanilla, NET};
    use std::ops::ControlFlow;

    let held = match hand {
        Hand::Main => game.stack_in(game.selected),
        Hand::Off => game.offhand.as_ref(),
    };
    if held.is_some_and(|stack| stack.item.as_ref() == NET) && can_catch_vanilla(game, index) {
        return ControlFlow::Break(true);
    }
    ControlFlow::Continue(())
}
