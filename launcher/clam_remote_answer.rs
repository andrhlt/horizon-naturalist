fn clam_remote_answer(game: &Game<'_>, index: usize, hand: Hand) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;

    let Some(mob) = game.mobs.get(index).filter(|mob| mob.kind.id == "naturalist:clam" && mob.alive()) else {
        return ControlFlow::Continue(());
    };
    let held = match hand {
        Hand::Main => game.stack_in(game.selected),
        Hand::Off => game.offhand.as_ref(),
    };
    if game.creative && held.is_some_and(|stack| stack.item.as_ref() == "minecraft:debug_stick") {
        return ControlFlow::Break(true);
    }
    let (_, open, treasure) = super::naturalist_deer_variants::clam_appearance(game, mob);
    if open && (held.is_some() || treasure && mob.extra.main_hand_stack().is_some()) {
        return ControlFlow::Break(true);
    }
    if held.is_some_and(|stack| stack.item.as_ref() == "minecraft:water_bucket") {
        return ControlFlow::Break(false);
    }
    ControlFlow::Continue(())
}
