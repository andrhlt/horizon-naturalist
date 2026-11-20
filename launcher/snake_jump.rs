fn snake_jump(mob: &Mob) -> std::ops::ControlFlow<f32> {
    if matches!(mob.kind.id.as_str(), "naturalist:snake" | "naturalist:rat") {
        return std::ops::ControlFlow::Break(0.0);
    }
    std::ops::ControlFlow::Continue(())
}
