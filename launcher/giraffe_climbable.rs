fn giraffe_climbable(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if (mob.kind.id == "naturalist:giraffe" || super::nav::naturalist_elephants::includes(&mob.kind.id)) {
        std::ops::ControlFlow::Break(false)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
