fn crab_set_tame(mob: &mut Mob, tame: bool) -> std::ops::ControlFlow<()> {
    if mob.kind.id == super::nav::naturalist_crab::KIND || matches!(mob.kind.id.as_str(), "naturalist:ant" | "naturalist:hedgehog") {
        mob.extra.tame = tame;
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
