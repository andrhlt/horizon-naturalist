fn tiger_tick(mob: &mut Mob) {
    use super::goals::naturalist_cat_goals as tiger;
    if tiger::is_big_cat(&mob.kind.id) {
        tiger::tick(mob);
    }
}
