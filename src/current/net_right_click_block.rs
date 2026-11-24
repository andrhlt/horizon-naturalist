fn capture_net_right_click_block(
    game: &mut Game<'_>,
    click: &BlockClick<'_>,
    _: &mut mod_api::TriState,
    _: &mut mod_api::TriState,
    result: &mut mod_api::InteractionResult,
    canceled: &mut bool,
) {
    let Some(stack) = game.held().cloned() else { return };
    if let Some(answer) = crate::game::naturalist_capture_net::release_at(game, &stack, click.pos) {
        *result = answer;
        *canceled = answer != mod_api::InteractionResult::Pass;
    }
}
