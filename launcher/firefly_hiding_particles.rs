fn firefly_hiding_particles(game: &mut Game<'_>, events: &mut Vec<MobEvent>) {
    use horizon_client::particles::{ParticleOptions, ParticleType};

    events.retain(|event| {
        let MobEvent::LevelParticles { kind: "naturalist:firefly_hiding", at, spread, speed, count } = event else {
            return true;
        };
        let grass = "minecraft:short_grass";
        let Some(state) = game.pack.assets.registry.default_state(grass) else {
            return false;
        };
        if game.serving.is_none() && game.options.dedicated.is_none() {
            let pos = at.map(|coord| coord.floor() as i32);
            if let Some((sprite, tint)) = game.particle_sprite(state, pos) {
                let options = ParticleOptions::block(ParticleType::Block, sprite, tint).with_source(grass);
                game.particles.send_particles(&options, *at, *spread, *speed, *count);
            }
        }
        let options = ParticleOptions::simple(ParticleType::Block);
        game.lan_level_particles(None, &options, *at, *spread, *speed, *count, Some(state));
        false
    });
}
