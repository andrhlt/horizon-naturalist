fn starfish_prevent_flop(mob: &Mob) -> std::ops::ControlFlow<()> {
    if matches!(mob.kind.id.as_str(), "naturalist:starfish" | "naturalist:clam") {
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
