fn elephant_inventory_open(game: &mut Game<'_>, index: usize) -> std::ops::ControlFlow<()> {
    use horizon_client::mobs::nav::naturalist_elephants;
    let mob = &game.mobs[index];
    if !naturalist_elephants::includes(&mob.kind.id) {
        return std::ops::ControlFlow::Continue(());
    }
    if game.lan_acting.is_some() {
        // The mount menu slot already carries a guest's request out of its turn.
        game.mount_menu.open = Some(Open {
            mob: mob.id,
            species: mob.species(),
            columns: 0,
            saddle: None,
            armor: None,
            items: Vec::new(),
            show_saddle: false,
            show_armor: false,
        });
    } else {
        game.open_local_naturalist_elephant_inventory(mob.id);
    }
    std::ops::ControlFlow::Break(())
}
