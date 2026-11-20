fn tortoise_jump_threshold(mob: &Mob) -> std::ops::ControlFlow<f64> {
    if mob.kind.id == "naturalist:tortoise" {
        return std::ops::ControlFlow::Break(0.4);
    }
    std::ops::ControlFlow::Continue(())
}
