fn big_cat_remote_answer(game: &Game<'_>, index: usize, hand: Hand) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    let Some(mob) = game.mobs.get(index).filter(|mob| mob.alive() && mob.extra.tame) else {
        return ControlFlow::Continue(());
    };
    // The Java cat consumes all interactions once tamed. A Rat only
    // consumes a non-food interaction for its owner or a net capture.
    match mob.kind.id.as_str() {
        "naturalist:lion" | "naturalist:tiger" => ControlFlow::Break(true),
        "naturalist:rat" => {
            let held = match hand {
                Hand::Main => game.stack_in(game.selected),
                Hand::Off => game.offhand.as_ref(),
            };
            let item = held.map_or("", |stack| stack.item.as_ref());
            let owns = game.remote.as_ref().is_some_and(|remote| {
                mob.extra.owner == Some(horizon_client::lan::playerdata::uuid_ints(remote.profile()))
            });
            let secondary = game.keys.held(horizon_client::actions::Action::Sneak)
                || game.keys.raw(&winit::keyboard::KeyCode::ShiftRight);
            if item == "naturalist:capture_net"
                || (owns && !secondary && !mob.kind.food.iter().any(|food| food == item))
            {
                ControlFlow::Break(true)
            } else {
                ControlFlow::Continue(())
            }
        }
        _ => ControlFlow::Continue(()),
    }
}
