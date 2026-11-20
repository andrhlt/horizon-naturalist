fn whale_dry_travel(_result: (), mob: &mut Mob) {
    if mob.kind.id == "naturalist:whale" && !mob.fluids.in_water && mob.on_ground {
        mob.velocity.x *= 0.3;
        mob.velocity.z *= 0.3;
    }
}
