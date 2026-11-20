fn ostrich_jump_pose(mob: &Mob) -> std::ops::ControlFlow<()> {
    if mob.kind.id == super::naturalist_ostrich::KIND {
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
