fn snail_nav_tick(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    if mob.kind.id == "naturalist:snail" && super::naturalist_snail_control::tick_navigation(mob) {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
