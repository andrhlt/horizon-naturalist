fn capture_net_right_click_block(
    game: &mut Game<'_>,
    click: &BlockClick<'_>,
    use_block: &mut mod_api::TriState,
    use_item: &mut mod_api::TriState,
    cancellation_result: &mut mod_api::InteractionResult,
    canceled: &mut bool,
) {
    let _ = (use_block, use_item);
    let Some(stack) = game.stack_in(game.selected).cloned() else { return };
    if let Some(answer) = crate::game::naturalist_capture_net::release_at(game, &stack, click.pos) {
        *cancellation_result = answer;
        *canceled = answer != mod_api::InteractionResult::Pass;
    }
}
