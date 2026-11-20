fn giraffe_ridden(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    let elephant = super::nav::naturalist_elephants::includes(&mob.kind.id);
    if mob.kind.id != "naturalist:giraffe" && !elephant {
        return std::ops::ControlFlow::Continue(());
    }
    if let Some(rider) = mob.rider {
        mob.previous_yaw = rider.yaw;
        mob.yaw = rider.yaw;
        mob.head_yaw = rider.yaw;
        mob.head_pitch = rider.pitch * 0.5;
        let delta = (rider.yaw - mob.body_yaw + 180.0).rem_euclid(360.0) - 180.0;
        mob.body_yaw += delta * 0.35;
        mob.ai.nav.stop();
        mob.set_speed(mob.movement_speed() as f32 * if elephant { 0.3 } else { 1.0 });
        mob.ai.xxa = rider.strafe * 0.5;
        mob.ai.zza = rider.forward;
    }
    std::ops::ControlFlow::Break(())
}
