fn ant_queen_use(game: &mut Game<'_>, stack: &ItemStack, pos: [i32; 3], claimed: &mut bool) {
    if !*claimed && stack.item.as_ref() == super::naturalist_ant_hill::QUEEN {
        super::naturalist_ant_hill::place_queen(game, pos);
        *claimed = true;
    }
}
