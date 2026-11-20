fn dragonfly_push(mob: &Mob) -> std::ops::ControlFlow<()> {
    if matches!(mob.kind.id.as_str(), "naturalist:hedgehog" | "naturalist:dragonfly") {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
