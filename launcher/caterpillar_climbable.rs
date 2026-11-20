fn caterpillar_climbable(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == "naturalist:caterpillar" {
        std::ops::ControlFlow::Break(
            *mob.ai
                .nav
                .__horizon_mixins_naturalist_caterpillar_climbing(),
        )
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
