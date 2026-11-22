fn elephant_inventory_click_fixture(
    game: &mut Game<'_>,
    words: &[&str],
) -> std::ops::ControlFlow<Result<Option<Value>>> {
    use horizon_client::menu::Input;
    use std::ops::ControlFlow;
    if words == ["elephant-menu-geometry"] && game.renderer.is_none()
        && game.naturalist_elephant_inventory().is_some()
    {
        let capture = |game: &Game<'_>| {
            let mut batch = crate::hud::HudBatch::new([640, 480]);
            let drawn = game.draw_native_screen(&mut batch, 0.0, 0.0);
            let inset = batch.vertices.iter().filter(|v| {
                !v.pos.iter().any(|c| !c.is_finite())
                    && v.text == 0
                    && (8.0..=77.0).contains(&v.pos[0])
                    && (18.0..=87.0).contains(&v.pos[1])
            }).map(|v| v.pos).collect::<Vec<_>>();
            (drawn, inset)
        };
        let saved_cursor = game.cursor;
        game.cursor = (18.0, 30.0);
        let (drawn, first) = capture(game);
        game.cursor = (69.0, 70.0);
        let (_, second) = capture(game);
        game.cursor = saved_cursor;
        return ControlFlow::Break(Ok(Some(serde_json::json!({
            "drawn": drawn,
            "inset_vertices": first.len(),
            "responds_to_cursor": first != second,
        }))));
    }
    let ["click-slot", slot, rest @ ..] = words else {
        return ControlFlow::Continue(());
    };
    if game.renderer.is_some() || game.naturalist_elephant_inventory().is_none() {
        return ControlFlow::Continue(());
    }
    let Ok(slot) = slot.parse::<usize>() else {
        return ControlFlow::Break(Err("expected an inventory slot".into()));
    };
    let input = if rest.contains(&"shift") { Input::QuickMove } else { Input::Pickup };
    let handled = game.menu_click(Some(slot), i32::from(rest.contains(&"right")), input);
    ControlFlow::Break(Ok(Some(serde_json::json!({ "ok": handled }))))
}
