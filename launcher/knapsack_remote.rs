fn knapsack_remote(game: &Game<'_>, index: usize, hand: Hand) -> std::ops::ControlFlow<bool> {
    use super::naturalist_knapsack as sack;
    use std::ops::ControlFlow;

    let stack = match hand {
        Hand::Main => game.stack_in(game.selected),
        Hand::Off => game.offhand.as_ref(),
    };
    if stack.is_some_and(|stack| stack.item.as_ref() == sack::ITEM && !sack::filled(stack))
        && game.mobs.get(index).is_some_and(sack::capturable)
    {
        return ControlFlow::Break(true);
    }
    ControlFlow::Continue(())
}
