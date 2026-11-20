fn shark_pushable(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if matches!(mob.kind.id.as_str(), super::nav::naturalist_shark_state::KIND | super::nav::naturalist_whale_state::KIND) {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
