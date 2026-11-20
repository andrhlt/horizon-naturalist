fn giraffe_pushable(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if (mob.kind.id == "naturalist:giraffe" || super::nav::naturalist_elephants::includes(&mob.kind.id)) && (mob.rider.is_some() || mob.extra.rider.is_some()) {
        std::ops::ControlFlow::Break(false)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
