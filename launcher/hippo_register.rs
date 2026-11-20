fn hippo_register(mob: &mut Mob) {
    if mob.kind.id == "naturalist:hippo" {
        super::goals::naturalist_hippo_goals::register(mob);
    }
}
