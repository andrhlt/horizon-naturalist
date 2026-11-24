fn capture_net_right_click_block(
    game: &mut Game<'_>,
    click: &BlockClick<'_>,
    _: &mut mod_api::TriState,
    _: &mut mod_api::TriState,
    result: &mut mod_api::InteractionResult,
    canceled: &mut bool,
) {
    // Let the client predict/pass the click; release_at intentionally returns
    // success without spawning on a remote game. Only the authoritative local
    // side may create the mob and consume the held item.
    if game.remote.is_some() { return; }
    let Some(stack) = game.held().cloned() else { return };
    if let Some(answer) = crate::game::naturalist_capture_net::release_at(game, &stack, click.pos) {
        *result = answer;
        *canceled = answer != mod_api::InteractionResult::Pass;
    }
}
