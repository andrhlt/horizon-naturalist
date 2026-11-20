fn shark_transient_speed(mob: &Mob, attribute: Attr) -> std::ops::ControlFlow<Option<(f64, Op)>> {
    if attribute == Attr::MovementSpeed
        && mob.kind.id == super::nav::naturalist_shark_state::KIND
        && mob.ai.target.is_some()
    {
        return std::ops::ControlFlow::Break(Some((0.4, Op::AddMultipliedTotal)));
    }
    std::ops::ControlFlow::Continue(())
}
