fn capture_net_right_click(
    game: &mut Game<'_>,
    _: &ItemClick<'_>,
    result: &mut mod_api::InteractionResult,
    canceled: &mut bool,
) {
    let Some(stack) = game.held().cloned() else { return };
    if let Some(answer) = crate::game::naturalist_capture_net::release(game, &stack) {
        *result = answer;
        *canceled = answer != mod_api::InteractionResult::Pass;
    }
}
