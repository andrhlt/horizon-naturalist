fn glow_goop_replaceable(behaviours: &Behaviours, state: BlockStateId) -> std::ops::ControlFlow<bool> {
    if behaviours.data(state).item.as_ref() == "naturalist:glow_goop" {
        std::ops::ControlFlow::Break(true)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
