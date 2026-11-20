fn vulture_experience(mob: &Mob) -> std::ops::ControlFlow<i32> {
    if matches!(
        mob.kind.id.as_str(),
        "naturalist:vulture" | "naturalist:dragonfly" | "naturalist:lizard_tail"
    ) {
        return std::ops::ControlFlow::Break(0);
    }
    std::ops::ControlFlow::Continue(())
}
