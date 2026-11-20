fn naturalist_sound_namespace(
    audio: &mut Audio, event: &str, at: Option<[f64; 3]>, volume: f32, pitch: f32, seed: i64,
) -> std::ops::ControlFlow<()> {
    // Stock mob voices prefix unqualified event names with minecraft:.
    if let Some(event) = event.strip_prefix("minecraft:").filter(|id| id.starts_with("naturalist:")) {
        audio.play_seeded(event, at, volume, pitch, seed);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
