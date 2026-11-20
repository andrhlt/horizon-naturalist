fn glow_goop_outline(game: &Game<'_>, state: BlockStateId) -> std::ops::ControlFlow<Option<[f64; 6]>> {
    use std::ops::ControlFlow;
    let id = naturalist_glow_goop::ID;
    if game.res.facts().get(state).name.as_ref() != id {
        return ControlFlow::Continue(());
    }
    let main = game.stack_in(game.selected);
    ControlFlow::Break(main.is_some_and(|stack| stack.item.as_ref() == id)
        .then_some([0.0, 0.0, 0.0, 1.0, 1.0, 1.0]))
}
