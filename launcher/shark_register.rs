fn shark_register(mob: &mut Mob) {
    if mob.kind.id == super::nav::naturalist_shark_state::KIND {
        super::goals::naturalist_shark_goals::register(mob);
    } else if mob.kind.id == super::nav::naturalist_whale_state::KIND {
        super::goals::naturalist_whale_goals::register(mob);
    }
}
