fn caterpillar_dims(mob: &Mob) -> std::ops::ControlFlow<(f64, f64, f64)> {
    if mob.kind.id != "naturalist:caterpillar" || !mob.is_baby() {
        return std::ops::ControlFlow::Continue(());
    }
    let scale = mob.model_scale();
    let scaled = |dimension: f64| f64::from(dimension as f32 * scale);
    let kind = &mob.kind;
    std::ops::ControlFlow::Break((
        scaled(kind.width),
        scaled(kind.height),
        scaled(kind.eye_height),
    ))
}
