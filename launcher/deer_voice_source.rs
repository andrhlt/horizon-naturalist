fn deer_voice_source(event: &str) -> std::ops::ControlFlow<usize> {
    let category = if event.starts_with("naturalist:entity.tortoise.egg_") { "block" } else { "neutral" };
    if event.starts_with("naturalist:entity.")
        && let Some(source) = SOUND_SOURCES.iter().position(|name| *name == category)
    {
        return std::ops::ControlFlow::Break(source);
    }
    std::ops::ControlFlow::Continue(())
}
