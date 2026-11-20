fn giraffe_controlled(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if (mob.kind.id == "naturalist:giraffe" || super::nav::naturalist_elephants::includes(&mob.kind.id)) {
        std::ops::ControlFlow::Break(mob.rider.is_some())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
