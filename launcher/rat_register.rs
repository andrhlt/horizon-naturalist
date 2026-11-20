fn rat_register(mob: &mut Mob) {
    use super::goals::naturalist_rat_goals as rat;
    if mob.kind.id == rat::KIND {
        rat::register(mob);
    }
}
