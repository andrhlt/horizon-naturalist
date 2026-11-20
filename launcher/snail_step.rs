fn snail_step(keep: bool, mob: &mut Mob, events: &mut Vec<MobEvent>) -> bool {
    if keep && mob.kind.id == "naturalist:snail" {
        super::naturalist_snail::tick(mob, events);
    }
    keep
}
