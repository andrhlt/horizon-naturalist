fn elephant_inventory_guest_menu(game: &Game<'_>, index: usize) -> std::ops::ControlFlow<Menu> {
    use super::native_hooks::naturalist_elephant_inventory::menu_kind;
    use horizon_client::{containers::mod_menu_type, mobs::nav::naturalist_elephants};
    let mob = &game.mobs[index];
    let Some(kind) = mod_menu_type(menu_kind(mob.extra.chested)).filter(|_| naturalist_elephants::includes(&mob.kind.id)) else {
        return std::ops::ControlFlow::Continue(());
    };
    let env = game.menu_env();
    let mut menu = Menu::new(Kind::Mod(kind));
    for (slot, stack) in mob.extra.items.iter().take(kind.slots).cloned().enumerate() {
        menu.set_slot(&env, slot, stack);
    }
    if !mob.extra.chested {
        for slot in 0..naturalist_elephants::CHEST_SLOTS {
            menu.hide_slot(slot);
        }
    }
    std::ops::ControlFlow::Break(menu)
}
