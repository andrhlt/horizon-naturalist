fn mole_wolf_pose(_result: (), mob: &Mob, render_state: &State, poses: &mut Poses) {
    if !super::goals::naturalist_mole_goals::state(&mob.ai.nav).wolf_digging {
        return;
    }
    let scratch = (render_state.age * 1.1).mcos() * 0.9;
    poses.xrot("right_front_leg", scratch);
    poses.xrot("left_front_leg", -scratch);
    let head = poses.get("head");
    head[1] += 1.5;
    head[3] = head[3].max(0.85);
    for (part, tilt) in [("body", 0.15), ("tail", -0.15), ("upper_body", 0.15)] {
        if poses.has(part) {
            let pose = poses.get(part);
            pose[1] += 1.5;
            pose[3] += tilt;
        }
    }
}
