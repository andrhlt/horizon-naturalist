fn whale_control(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    use super::nav::naturalist_whale_state::KIND;
    use std::ops::ControlFlow;
    if mob.kind.id != KIND {
        return ControlFlow::Continue(());
    }
    if !mob.ai.mov.has_wanted() {
        mob.ai.speed = 0.0;
        mob.ai.xxa = 0.0;
        mob.ai.yya = 0.0;
        mob.ai.zza = 0.0;
        return ControlFlow::Break(());
    }
    mob.ai.mov.operation = Operation::Wait;
    let [x, y, z] = mob.ai.mov.wanted;
    let (dx, dy, dz) = (x - mob.position.x, y - mob.position.y, z - mob.position.z);
    let horizontal = dx.hypot(dz);
    if horizontal * horizontal + dy * dy < f64::from(2.500_000_3e-7_f32) {
        mob.ai.zza = 0.0;
        return ControlFlow::Break(());
    }
    mob.yaw = rotlerp(mob.yaw, dz.atan2(dx).to_degrees() as f32 - 90.0, 4.0);
    mob.body_yaw = mob.yaw;
    mob.head_yaw = mob.yaw;
    let speed = (mob.ai.mov.speed_modifier * f64::from(mob.movement_speed())) as f32;
    if mob.fluids.in_water {
        mob.ai.speed = speed * 0.02;
        let pitch = (-(dy.atan2(horizontal).to_degrees() as f32)).clamp(-45.0, 45.0);
        mob.head_pitch = rotlerp(mob.head_pitch, pitch, 5.0);
        let (sin, cos) = pitch.to_radians().sin_cos();
        mob.ai.yya = -sin * speed;
        mob.ai.zza = cos * speed;
    } else {
        mob.ai.speed = speed * 0.1;
    }
    ControlFlow::Break(())
}
