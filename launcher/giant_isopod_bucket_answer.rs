fn giant_isopod_bucket_answer(game: &Game<'_>, index: usize, hand: Hand) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;

    if !game.mobs.get(index).is_some_and(|mob| mob.kind.id == "naturalist:giant_isopod" && mob.alive()) {
        return ControlFlow::Continue(());
    }
    let stack = match hand {
        Hand::Main => game.stack_in(game.selected),
        Hand::Off => game.offhand.as_ref(),
    };
    match stack.map(|stack| stack.item.as_ref()) {
        Some("minecraft:water_bucket") => ControlFlow::Break(true),
        Some("minecraft:bucket") => ControlFlow::Break(false),
        _ => ControlFlow::Continue(()),
    }
}
