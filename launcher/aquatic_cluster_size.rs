fn aquatic_cluster_size() -> std::ops::ControlFlow<u32> {
    match naturalist_aquatic_spawn_limits::limit() {
        Some(limit) => std::ops::ControlFlow::Break(limit),
        None => std::ops::ControlFlow::Continue(()),
    }
}
