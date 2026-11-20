fn elephant_inventory_click(game: &mut Game<'_>, right: bool) -> std::ops::ControlFlow<()> {
    use horizon_client::{hud::HudBatch, menu::Input};
    use std::ops::ControlFlow;
    if game.remote.is_some() || game.naturalist_elephant_inventory().is_none() {
        return ControlFlow::Continue(());
    }
    let Some(renderer) = game.renderer.as_ref() else {
        return ControlFlow::Break(());
    };
    let batch = HudBatch::new([renderer.config.width, renderer.config.height]);
    if let Some(slot) = game.slot_at(&batch, game.cursor.0, game.cursor.1) {
        let input = if game.has_shift_down() { Input::QuickMove } else { Input::Pickup };
        game.menu_click(Some(slot), i32::from(right), input);
    }
    ControlFlow::Break(())
}
