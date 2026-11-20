fn butterfly_age(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == "naturalist:butterfly" {
        std::ops::ControlFlow::Break(false)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
