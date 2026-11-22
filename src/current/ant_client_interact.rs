fn ant_client_interact(
    game: &mut Game<'_>,
    index: usize,
    hand: Hand,
) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    let Some(mob) = game.mobs.get(index) else {
        return ControlFlow::Continue(());
    };
    let empty = match hand {
        Hand::Main => game.stack_in(game.selected).is_none(),
        Hand::Off => game.offhand.is_none(),
    };
    if mob.kind.id == "naturalist:ant" && empty && mob.extra.leash.is_none() {
        return ControlFlow::Break(false);
    }
    ControlFlow::Continue(())
}
