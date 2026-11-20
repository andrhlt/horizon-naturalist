fn duck_xp(mob: &mut Mob) -> std::ops::ControlFlow<i32> {
    if mob.kind.id == "naturalist:duck" {
        return std::ops::ControlFlow::Break(10);
    }
    std::ops::ControlFlow::Continue(())
}
