fn snail_portal(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == "naturalist:snail" {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
