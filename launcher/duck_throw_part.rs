fn duck_throw_part(game: &mut Game<'_>, stack: &ItemStack) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client_render::audio::naturalist_duck_throw::DuckThrow;
    use std::ops::ControlFlow;

    if stack.item.as_ref() != "naturalist:duck_egg" {
        return ControlFlow::Continue(());
    }
    let Some(feet) = game.player.as_ref().map(|player| player.position) else {
        return ControlFlow::Continue(());
    };
    let Some(_scope) = DuckThrow::enter([feet.x, feet.y, feet.z]) else {
        return ControlFlow::Continue(());
    };
    ControlFlow::Break(game.throw_item(stack))
}
