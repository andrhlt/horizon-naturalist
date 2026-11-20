fn butterfly_nectar_step(keep: bool, mob: &mut Mob) -> bool {
    if keep {
        super::goals::naturalist_insect_goals::finish_butterfly_step(mob);
    }
    keep
}
