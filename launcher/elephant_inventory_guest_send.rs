fn elephant_inventory_guest_send(
    game: &Game<'_>,
    host: &mut LanHost,
    conn: u32,
) -> std::ops::ControlFlow<()> {
    use horizon_client::mobs::nav::naturalist_elephants;
    use std::ops::ControlFlow;

    let Some(mount) = host.play.get_mut(&conn).and_then(|guest| guest.mount_window.as_mut()) else {
        return ControlFlow::Continue(());
    };
    let Some(mob) = game.mobs.iter().find(|mob| mob.id == mount.mob
        && naturalist_elephants::includes(&mob.kind.id)) else {
        return ControlFlow::Continue(());
    };
    let Some(guest) = host.hub.guests.iter().find(|guest| guest.conn == conn) else {
        return ControlFlow::Break(());
    };
    let mut slots = mob.extra.items.clone();
    slots.resize(naturalist_elephants::INVENTORY_SLOTS, None);
    slots.extend((9..36).chain(0..9).map(|slot| guest.inventory[slot].clone()));
    let now = Some((slots, guest.carried.clone()));
    if mount.sent == now {
        return ControlFlow::Break(());
    }
    mount.state = (mount.state + 1) & 0x7fff;
    let (slots, carried) = now.as_ref().unwrap();
    let packet = C::ContainerSetContent(play::ContainerSetContent {
        container: mount.window,
        state: mount.state,
        items: slots.iter().map(|stack| host.hub.stack(stack)).collect(),
        carried: host.hub.stack(carried),
    });
    mount.sent = now;
    host.hub.send_to(conn, &packet);
    ControlFlow::Break(())
}
