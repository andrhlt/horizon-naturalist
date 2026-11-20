fn snail_push(mob: &Mob) -> std::ops::ControlFlow<()> {
    if mob.kind.id == "naturalist:snail" {
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
