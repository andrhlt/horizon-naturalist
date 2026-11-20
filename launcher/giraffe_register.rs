fn giraffe_register(mob: &mut Mob) {
    if mob.kind.id == super::naturalist_giraffe::KIND {
        super::naturalist_giraffe::register(mob);
    }
}
