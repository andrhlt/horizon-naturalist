fn crab_health_base(mob: &Mob, attribute: Attr) -> std::ops::ControlFlow<f64> {
    if matches!(mob.kind.id.as_str(), super::nav::naturalist_crab::KIND | super::naturalist_hedgehog::KIND)
        && attribute == Attr::MaxHealth
    {
        return std::ops::ControlFlow::Break(base_value(mob, attribute));
    }
    std::ops::ControlFlow::Continue(())
}
