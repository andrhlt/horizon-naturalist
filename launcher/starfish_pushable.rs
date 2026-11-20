fn starfish_pushable(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if matches!(mob.kind.id.as_str(), "naturalist:starfish" | "naturalist:clam") {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
