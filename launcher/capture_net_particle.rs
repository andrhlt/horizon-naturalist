fn capture_net_particle(
    engine: &ParticleEngine,
    options: &ParticleOptions,
    position: [f64; 3],
    seed: i64,
) -> std::ops::ControlFlow<Option<Particle>> {
    use std::ops::ControlFlow;

    let name = "naturalist:capture_net_swing";
    if options.mod_particle.as_deref() != Some(name) {
        return ControlFlow::Continue(());
    }
    let Some((sprites, _)) = engine.mod_sets.get(name) else {
        return ControlFlow::Break(None);
    };
    let mut random = LegacyRandom::new(seed);
    let mut particle = Particle::at(&mut random, ParticleType::SweepAttack, position);
    particle.random = random;
    particle.set = Some(*sprites);
    particle.lifetime = 4;
    particle.size = 0.8;
    particle.gravity = 0.0;
    particle.friction = 1.0;
    particle.velocity = [0.0; 3];
    particle.has_physics = false;
    particle.light = Light::FullBright;
    particle.behaviour = Behaviour::Animated;
    engine.sprite_from_age(&mut particle);
    ControlFlow::Break(Some(particle))
}
