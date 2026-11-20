fn insect_portals(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if matches!(
        mob.kind.id.as_str(),
        "naturalist:ant" | "naturalist:hedgehog" | "naturalist:butterfly" | "naturalist:caterpillar"
    ) {
        std::ops::ControlFlow::Break(false)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
