fn elephant_inventory_remote_screen(
    game: &mut Game<'_>,
    key: Pos,
    menu: &str,
    extra: &Option<horizon_protocol::menu_extra::Message>,
) -> std::ops::ControlFlow<()> {
    use self::naturalist_elephant_inventory::{MENU, UNCHESTED};
    if !matches!(menu, MENU | UNCHESTED) {
        return std::ops::ControlFlow::Continue(());
    }
    if let Some(extra) = extra {
        game.show_naturalist_elephant_inventory(key, extra.clone());
    }
    std::ops::ControlFlow::Break(())
}
