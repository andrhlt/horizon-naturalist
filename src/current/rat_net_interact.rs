fn rat_net_interact(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use std::ops::ControlFlow;
    let Some(index) = game.mob_interaction_target(target) else { return ControlFlow::Continue(()) };
    if game.mobs[index].kind.id != "naturalist:rat"
        || game.held().is_none_or(|s| s.item.as_ref() != "naturalist:capture_net")
        || game.remote.is_some()
    {
        return ControlFlow::Continue(());
    }
    let id = game.mobs[index].id as i32;
    if crate::game::naturalist_capture_net::capture(game, id) {
        return ControlFlow::Break(Ok(true));
    }
    ControlFlow::Continue(())
}
