fn caterpillar_motion(keep: bool, mob: &mut Mob) -> bool {
    if keep && mob.kind.id == "naturalist:caterpillar" {
        *mob.ai
            .nav
            .__horizon_mixins_naturalist_caterpillar_climbing_mut() = mob.horizontal_collision;
        if mob.horizontal_collision {
            mob.velocity.y *= 0.5;
        }
    }
    keep
}
