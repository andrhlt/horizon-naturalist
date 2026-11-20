use super::{Game, ItemStack};

pub(super) const ITEM: &str = "naturalist:knapsack";

pub(super) fn filled(stack: &ItemStack) -> bool {
    stack.custom_data().is_some_and(|tag| tag.contains_key(&"id".into()))
}

pub(super) fn capturable(mob: &horizon_client::mobs::Mob) -> bool {
    mob.alive() && !mob.removed && mob.extra.mount.is_none() && mob.is_baby()
}

pub(super) fn replace_one(game: &mut Game<'_>, source: &ItemStack, replacement: ItemStack) {
    if source.count <= 1 {
        *game.held_mut() = Some(replacement);
        return;
    }
    if let Some(stack) = game.held_mut().as_mut() {
        stack.count -= 1;
    }
    if let Some(overflow) = game.give(replacement) && let Some(player) = game.player.as_ref() {
        game.spawn_item_own(overflow, player.position.components(), 0);
    }
}
