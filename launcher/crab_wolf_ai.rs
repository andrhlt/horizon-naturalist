fn crab_wolf_ai(mob: &Mob) -> std::ops::ControlFlow<()> {
    if mob.kind.id == super::nav::naturalist_crab::KIND {
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
