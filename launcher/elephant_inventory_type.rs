fn elephant_inventory_type(game: super::native_hooks::naturalist_elephant_inventory::GameView<'_>) -> std::ops::ControlFlow<Option<&str>> {
    use std::ops::ControlFlow;

    if game.remote.is_none() && let Some((_, chested)) = game.naturalist_elephant_inventory() {
        return ControlFlow::Break(Some(super::native_hooks::naturalist_elephant_inventory::menu_kind(chested)));
    }
    ControlFlow::Continue(())
}
