fn giraffe_pose(mob: &mut Mob) {
    if mob.kind.id == "naturalist:giraffe" {
        mob.extra.sprinting = mob.ai.mov.has_wanted() && mob.ai.mov.speed_modifier >= 1.3;
    }
}
