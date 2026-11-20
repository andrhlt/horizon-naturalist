fn voice_level_fixture(audio: &mut Audio, event: &str, volume: f32) {
    if !event.starts_with("naturalist:entity.") || !event.ends_with(".hurt")
        || std::env::var_os("NATURALIST_VOICE_TRACE").is_none()
    {
        return;
    }
    let source = audio.source_override.unwrap_or_else(|| {
        crate::settings::sound_source(&crate::sound_events::normalize(event))
    });
    let category = crate::settings::SOUND_SOURCES.get(source).copied().unwrap_or("unknown");
    eprintln!("[naturalist-voice] {event} {volume} {category}");
}
