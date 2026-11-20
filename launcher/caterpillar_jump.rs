fn caterpillar_jump(mob: &Mob) -> std::ops::ControlFlow<f32> {
    if matches!(mob.kind.id.as_str(), "naturalist:ant" | "naturalist:caterpillar") {
        std::ops::ControlFlow::Break(0.0)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
