fn firefly_peaceful_placement() -> std::ops::ControlFlow<bool> {
    if super::naturalist_deer_placement::firefly_active() {
        std::ops::ControlFlow::Break(false)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
