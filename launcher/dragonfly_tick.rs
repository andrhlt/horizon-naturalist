fn dragonfly_tick(keep: bool, mob: &mut Mob) -> bool {
    if keep && mob.kind.id == "naturalist:dragonfly" {
        mob.velocity.y *= 0.6;
    }
    keep
}
