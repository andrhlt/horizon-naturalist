fn clam_finalized(mob: &mut Mob) {
    use crate::{items::ItemStack, mobs::nav::naturalist_clam as clam};

    if mob.kind.id != clam::KIND || mob.random.next_float() >= 0.05 {
        return;
    }
    let item = match mob.random.next_int_bound(125).unwrap_or(0) {
        0..5 => "minecraft:heart_of_the_sea",
        5..35 => "minecraft:golden_apple",
        35..75 => "minecraft:ender_pearl",
        _ => "naturalist:capture_net",
    };
    clam::state_mut(&mut mob.ai.nav).has_treasure = true;
    mob.extra.set_main_hand_stack(Some(ItemStack::new(item, 1)));
}
