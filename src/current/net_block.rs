fn capture_net_block(
    game: &mut Game<'_>,
    _: &BlockClick<'_>,
    phase: UsePhase,
    result: &mut mod_api::InteractionResult,
    canceled: &mut bool,
) {
    if phase != UsePhase::ItemBeforeBlock { return; }
    let Some(stack) = game.held().cloned() else { return };
    if let Some(answer) = crate::game::naturalist_capture_net::release(game, &stack) {
        *result = answer;
        *canceled = answer != mod_api::InteractionResult::Pass;
    }
}
