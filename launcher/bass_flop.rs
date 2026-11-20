fn bass_flop(mob: &mut Mob, event: &'static str, events: &mut Vec<MobEvent>) -> std::ops::ControlFlow<()> {
    use std::ops::ControlFlow;
    let sound = match (mob.kind.id.as_str(), event) {
        ("naturalist:bass", "entity.cod.flop") => "naturalist:entity.bass.flop",
        ("naturalist:catfish" | "naturalist:ray" | "naturalist:anglerfish", "entity.cod.flop") => "naturalist:entity.catfish.flop",
        ("naturalist:piranha", "entity.cod.flop") => "naturalist:entity.piranha.flop",
        ("naturalist:blobfish", "entity.cod.flop") => "naturalist:entity.blobfish.flop",
        _ => return ControlFlow::Continue(()),
    };
    let pitch = mob.voice_pitch();
    if !mob.silent {
        events.push(MobEvent::SoundId(sound.to_owned(),
            mob.position.components(), ai::sound_volume(mob), pitch));
    }
    ControlFlow::Break(())
}
