fn snail_shell_drops(game: &Game<'_>, context: &NativeLootDrops<'_>, drops: &mut Vec<ItemStack>) {
    if context.player.is_none()
        && let Some((pos, state)) = context.block
    {
        super::naturalist_snail_shell::drops(game, pos, state, drops);
    }
}
