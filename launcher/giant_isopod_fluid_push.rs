fn giant_isopod_fluid_push(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == super::nav::naturalist_giant_isopod::KIND {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
