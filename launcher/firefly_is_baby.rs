fn firefly_is_baby(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == "naturalist:firefly" {
        return std::ops::ControlFlow::Break(mob.extra.age < 0);
    }
    std::ops::ControlFlow::Continue(())
}
