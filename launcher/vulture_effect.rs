fn vulture_effect(mob: &Mob, effect: &'static str) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == "naturalist:vulture" && matches!(effect, "hunger" | "minecraft:hunger") {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
