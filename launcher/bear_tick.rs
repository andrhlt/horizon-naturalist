fn bear_tick(mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) {
    if super::naturalist_bears::is_bear(&mob.kind.id) && mob.alive() {
        super::naturalist_bears::server_tick(mob, world, events);
    }
}
