fn capture_net_interact(game: &mut Game<'_>, entity: i32, stack: &ItemStack, result: &mut Option<mod_api::InteractionResult>) {
    if stack.item.as_ref() == "naturalist:capture_net" && result.is_none() && game.remote.is_none() && crate::game::naturalist_capture_net::capture(game, entity) {
        *result = Some(mod_api::InteractionResult::SuccessServer);
    }
}
