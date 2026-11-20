fn snail_shell_fill(game: &mut Game<'_>, stack: &ItemStack, pos: [i32; 3], face: [i32; 3], claimed: &mut bool) {
    if !*claimed && (super::naturalist_glow_goop::stack_layer(game, stack, pos)
        || super::naturalist_ostrich_game::place_egg_against_egg(game, stack, pos, face)
        || super::naturalist_snail_shell::fill(game, stack, pos))
    {
        *claimed = true;
    }
}
