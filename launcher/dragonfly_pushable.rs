fn dragonfly_pushable(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == "naturalist:dragonfly" {
        std::ops::ControlFlow::Break(false)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
