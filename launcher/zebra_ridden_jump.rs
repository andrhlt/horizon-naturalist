fn zebra_ridden_jump(mob: &mut Mob) {
    if mob.kind.id == super::naturalist_zebra::KIND {
        mob.extra.vector[0] = mob.attrs.jump_strength;
    }
}
