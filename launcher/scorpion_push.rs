fn scorpion_push(mob: &Mob) -> std::ops::ControlFlow<()> {
    if matches!(mob.kind.id.as_str(), "naturalist:desert_scorpion" | "naturalist:lizard") {
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
