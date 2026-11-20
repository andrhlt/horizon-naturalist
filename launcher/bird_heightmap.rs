fn bird_heightmap(level: &GameLevel<'_, '_>, kind: Heightmap, x: i32, z: i32) -> std::ops::ControlFlow<i32> {
    if kind == Heightmap::MotionBlockingNoLeaves && super::naturalist_deer_placement::is_bird_generation() {
        return std::ops::ControlFlow::Break(NaturalLevel::heightmap(level, Heightmap::MotionBlocking, x, z));
    }
    std::ops::ControlFlow::Continue(())
}
