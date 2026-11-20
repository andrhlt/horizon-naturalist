fn clam_persistence(mob: &Mob) -> std::ops::ControlFlow<bool> {
    use super::nav::naturalist_clam as clam;
    if mob.kind.id == clam::KIND && clam::state(&mob.ai.nav).has_treasure {
        return std::ops::ControlFlow::Break(true);
    }
    std::ops::ControlFlow::Continue(())
}
