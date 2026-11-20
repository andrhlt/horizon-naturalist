fn bird_flap_audio(flapped: bool, mob: &Mob, events: &mut Vec<MobEvent>) -> bool {
    if flapped
        && !mob.silent
        && mob.kind.id == "naturalist:bird"
        && let Some(event) = events.last_mut()
        && let MobEvent::Sound("entity.parrot.fly", at, volume, pitch) = event
    {
        *event = MobEvent::SoundId("naturalist:entity.bird.fly".to_owned(), *at, *volume, *pitch);
    }
    flapped
}
