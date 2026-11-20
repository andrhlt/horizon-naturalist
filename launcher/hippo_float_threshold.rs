fn hippo_float_threshold(mob: &Mob) -> std::ops::ControlFlow<f64> {
    if mob.kind.id == "naturalist:hippo" {
        std::ops::ControlFlow::Break(if mob.is_baby() {
            f64::from(0.2_f32)
        } else {
            1.0
        })
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
