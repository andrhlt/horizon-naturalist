fn whale_particle_events(game: &mut Game<'_>, events: &mut Vec<MobEvent>) {
    use horizon_client::particles::{ParticleOptions, ParticleType};
    events.retain(|event| {
        let MobEvent::Particles { kind, at, count } = event else { return true };
        let (particle, spread, speed) = match *kind {
            "naturalist:whale_cloud" => ("minecraft:cloud", [0.15, 0.35, 0.15], 0.12),
            "naturalist:whale_bubbles" => ("minecraft:bubble_column_up", [0.15, 0.25, 0.15], 0.0),
            "naturalist:whale_splash" => ("minecraft:splash", [2.5, 0.4, 2.5], 0.2),
            _ => return true,
        };
        let Some(particle) = ParticleType::from_name(particle) else { return false };
        let options = ParticleOptions::simple(particle);
        if game.serving.is_none() && game.options.dedicated.is_none() {
            game.particles.send_particles(&options, *at, spread, speed, *count);
        }
        game.native_send_particles_in_place(&options, None, *at, spread, speed, *count as i32);
        false
    });
}
