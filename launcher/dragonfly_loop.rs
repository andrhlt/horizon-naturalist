use super::{Attenuation, InstanceId, NEUTRAL, SoundInstance, Sounds, Ticking};

pub fn update(sounds: &mut impl Sounds, sources: impl Iterator<Item = (u32, [f64; 3], f32, bool)>) {
    let mut playing = std::mem::take(sounds.engine().__horizon_mixins_naturalist_dragonfly_sounds_mut());
    for (entity, position, speed, silent) in sources {
        // Java offers the loop once when tracking starts. A silent dragonfly
        // keeps its entry until it is removed, so changing Silent back cannot
        // restart an instance that was declined or stopped.
        let (handle, seen) = playing
            .entry(entity)
            .or_insert_with(|| ((!silent).then(|| start(sounds, position)), false));
        *seen = true;
        if silent {
            stop(sounds, handle.take());
        }
        if let Some(instance) = handle.and_then(|handle| sounds.engine().instance_mut(handle)) {
            instance.position = position;
            if speed >= 0.01 {
                instance.volume = speed.clamp(0.0, 0.5) * 1.2;
                instance.pitch = 1.1 + speed.clamp(1.1, 1.5) * (1.5 - 1.1);
            } else {
                instance.volume = 0.0;
                instance.pitch = 0.0;
            }
        }
    }
    playing.retain(|_, (handle, seen)| {
        if std::mem::take(seen) {
            true
        } else {
            stop(sounds, *handle);
            false
        }
    });
    *sounds.engine().__horizon_mixins_naturalist_dragonfly_sounds_mut() = playing;
}

fn start(sounds: &mut impl Sounds, position: [f64; 3]) -> InstanceId {
    let mut instance = SoundInstance::new(
        "naturalist:entity.dragonfly.loop",
        NEUTRAL,
        0.0,
        1.0,
        crate::random_source::create(),
        true,
        0,
        Attenuation::Linear,
        position,
        false,
    );
    instance.can_start_silent = true;
    instance.ticking = Ticking::Native;
    sounds.play_instance(instance, true).0
}

fn stop(sounds: &mut impl Sounds, handle: Option<InstanceId>) {
    if let Some(handle) = handle {
        sounds.stop_instance(handle);
        sounds.release(handle);
    }
}
