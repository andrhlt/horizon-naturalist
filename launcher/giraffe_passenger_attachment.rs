fn giraffe_passenger_attachment(mob: &Mob) -> std::ops::ControlFlow<[f64; 3]> {
    if mob.kind.id == "naturalist:giraffe" {
        std::ops::ControlFlow::Break([0.0, f64::from(mob.height()) * 0.6, 0.0])
    } else if super::nav::naturalist_elephants::includes(&mob.kind.id) {
        let [x, y, z] = super::nav::naturalist_elephants::seat(mob);
        // The engine normally rotates an attachment with its mount. Java's
        // override directly supplies a world-aligned offset for these two.
        let angle = f64::from(mob.yaw).to_radians();
        let (sin, cos) = angle.sin_cos();
        std::ops::ControlFlow::Break([x * cos + z * sin, y, z * cos - x * sin])
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
