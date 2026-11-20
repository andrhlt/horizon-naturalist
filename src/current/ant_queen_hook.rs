fn ant_queen_use(
    game: &mut Game<'_>,
    call: mod_api::__private::UseOnCall<'_>,
    original: impl FnOnce(&mut Game<'_>, mod_api::__private::UseOnCall<'_>) -> (),
) {
    if game.remote.is_none()
        && game
            .held()
            .is_some_and(|stack| stack.item.as_ref() == "naturalist:queen_ant")
    {
        let placed = crate::game::naturalist_ant_queen::place_queen(game, call.pos);
        *call.result = Some(if placed {
            mod_api::InteractionResult::Consume
        } else {
            mod_api::InteractionResult::Fail
        });
    } else {
        original(game, call);
    }
}
