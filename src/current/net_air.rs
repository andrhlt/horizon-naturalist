fn capture_net_air(game: &mut Game<'_>, call: mod_api::__private::UseItemCall<'_>, original: impl FnOnce(&mut Game<'_>, mod_api::__private::UseItemCall<'_>) -> ()) {
    let Some(stack) = game.held().cloned() else { original(game, call); return };
    if stack.item.as_ref() == "naturalist:hedgehog" {
        // A right click with a block target is a release; an empty ray is a throw.
        if game.pick().is_some()
            && let Some(answer) = crate::game::naturalist_capture_net::release(game, &stack)
        {
            *call.result = Some(answer);
            return;
        }
        if crate::game::naturalist_capture_net::throw_hedgehog(game) {
            *call.result = Some(mod_api::InteractionResult::SuccessServer);
            return;
        }
    }
    if stack.item.as_ref() == "naturalist:capture_net" && game.remote.is_none() {
        crate::game::naturalist_capture_net::swing(game);
    }
    if let Some(answer) = crate::game::naturalist_capture_net::release(game, &stack) {
        *call.result = Some(answer);
        return;
    }
    original(game, call);
}
