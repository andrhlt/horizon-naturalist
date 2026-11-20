fn turkey_egg(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    if mob.kind.id == "naturalist:turkey" {
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
