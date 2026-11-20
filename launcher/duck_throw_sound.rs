fn duck_throw_sound(audio: &mut Audio, event: &str, volume: f32, pitch: f32, seed: i64)
    -> std::ops::ControlFlow<()> {
    if event == "minecraft:entity.snowball.throw"
        && let Some(position) = naturalist_duck_throw::position()
    {
        audio.play_seeded("minecraft:entity.egg.throw", Some(position), volume, pitch, seed);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
