fn alligator_register(mob: &mut Mob) {
    use super::goals::naturalist_alligator_goals as gator;
    if mob.kind.id == gator::KIND {
        gator::register(mob);
    }
}
