fn crab_persistent(mob: &Mob) -> std::ops::ControlFlow<bool> {
    use super::nav::naturalist_crab as crab;

    if mob.kind.id == crab::KIND && crab::state(&mob.ai.nav).from_hand {
        return std::ops::ControlFlow::Break(true);
    }
    std::ops::ControlFlow::Continue(())
}
