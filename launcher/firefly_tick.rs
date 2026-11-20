fn firefly_tick(keep: bool, mob: &mut Mob, world: &MobWorld, random: &mut LegacyRandom, events: &mut Vec<MobEvent>) -> bool {
    if keep && mob.kind.id == "naturalist:firefly" {
        super::goals::naturalist_firefly_behavior::step(mob, world, random, events);
    }
    keep
}
