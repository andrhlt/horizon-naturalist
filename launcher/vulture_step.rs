fn vulture_step(mob: &Mob) -> std::ops::ControlFlow<Option<(&'static str, f32, f32)>> {
    if matches!(
        mob.kind.id.as_str(),
        "naturalist:vulture" | "naturalist:butterfly" | "naturalist:dragonfly"
    ) {
        return std::ops::ControlFlow::Break(None);
    }
    std::ops::ControlFlow::Continue(())
}
