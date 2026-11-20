fn snail_tick(mob: &mut Mob, world: &MobWorld<'_>, _events: &mut Vec<MobEvent>) {
    if mob.kind.id == "naturalist:snail" {
        super::naturalist_snail::check_hiding(mob, world);
    }
}
