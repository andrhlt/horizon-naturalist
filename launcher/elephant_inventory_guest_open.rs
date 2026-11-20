fn elephant_inventory_guest_open(
    game: &mut Game<'_>,
    host: &mut LanHost,
    conn: u32,
    mob: u32,
) -> std::ops::ControlFlow<()> {
    use horizon_client::{
        lan::{data::component_nbt, menu::Slots},
        mobs::nav::naturalist_elephants::{self, INVENTORY_SLOTS},
        text::Text,
    };
    use super::native_hooks::naturalist_elephant_inventory::menu_kind;
    use std::ops::ControlFlow;

    let Some(entity) = game.mobs.iter().find(|entity| entity.id == mob) else {
        return ControlFlow::Continue(());
    };
    if !naturalist_elephants::includes(&entity.kind.id) {
        return ControlFlow::Continue(());
    }
    let Some(gi) = host.hub.guests.iter().position(|guest| guest.conn == conn) else {
        return ControlFlow::Break(());
    };
    let menu_kind = menu_kind(entity.extra.chested);
    if !game.lan_mount_valid(&host.hub.guests[gi], mob) || !game.guest_may_open(host, conn, menu_kind, None) {
        return ControlFlow::Break(());
    }
    let entity = game.mobs.iter().find(|entity| entity.id == mob).unwrap();
    let [x, y, z] = entity.position.components().map(|n| n.floor() as i32);
    let [a, b, c, d] = mob.to_be_bytes();
    let data = vec![a, b, c, d, u8::from(entity.extra.chested)];
    let mut items = entity.extra.items.clone();
    items.resize(INVENTORY_SLOTS, None);
    let translation = if entity.kind.id == naturalist_elephants::MAMMOTH {
        "entity.naturalist.mammoth"
    } else {
        "entity.naturalist.elephant"
    };
    let title = entity.custom_name_text.clone()
        .or_else(|| entity.custom_name.clone().map(Text::literal))
        .unwrap_or_else(|| Text::translate(translation, &[]));
    let title = component_nbt(&title.component());
    game.guest_close_all(host, conn);
    host.hub.open_container(conn, menu_kind, title, (x, y, z), Slots::Native, items, Some(data));
    let Some(guest) = host.hub.guests.iter_mut().find(|guest| guest.conn == conn) else {
        return ControlFlow::Break(());
    };
    let Some(menu) = guest.menu.take() else { return ControlFlow::Break(()) };
    host.play.entry(conn).or_default().mount_window = Some(GuestMount {
        window: menu.window,
        mob,
        columns: 0,
        state: menu.state,
        sent: None,
    });
    game.lan_send_mount(host, conn);
    ControlFlow::Break(())
}
