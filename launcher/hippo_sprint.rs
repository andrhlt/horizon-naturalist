fn hippo_sprint(mob: &mut Mob) {
    if mob.kind.id == "naturalist:hippo" {
        let moving = mob.ai.mov.operation == super::control::Operation::MoveTo;
        let sprint = moving && mob.ai.mov.speed_modifier >= 1.0;
        if mob.extra.sprinting != sprint {
            mob.extra.sprinting = sprint;
        }
    }
}
