fn snail_nav_update(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == "naturalist:snail" {
        std::ops::ControlFlow::Break(
            mob.on_ground
                || mob.is_in_liquid()
                || super::naturalist_snail::snail(mob).surface.attached,
        )
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
