fn elephant_inventory_guest_store(
    game: &mut Game<'_>,
    index: usize,
    menu: &Menu,
) -> std::ops::ControlFlow<()> {
    use horizon_client::mobs::nav::naturalist_elephants;
    let mob = &mut game.mobs[index];
    if !naturalist_elephants::includes(&mob.kind.id) {
        return std::ops::ControlFlow::Continue(());
    }
    let items = (0..naturalist_elephants::INVENTORY_SLOTS)
        .map(|slot| menu.slot(slot).cloned()).collect();
    super::native_hooks::naturalist_elephant_inventory::naturalist_elephant_contents(mob, items);
    game.world.modified.insert((mob.position.x.floor() as i32 >> 4,
                                mob.position.z.floor() as i32 >> 4));
    std::ops::ControlFlow::Break(())
}
