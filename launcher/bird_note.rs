fn bird_note(game: &mut Game<'_>) {
    use horizon_client::{
        mobs::nav::naturalist_bird_voice::take_note,
        particles::{ParticleOptions, ParticleType},
    };
    use horizon_core::random::RandomSource;
    let mut pending = Vec::new();
    for mob in game.mobs.iter_mut().filter(|mob| mob.kind.id == "naturalist:bird") {
        if let Some(at) = take_note(&mut mob.ai.nav) {
            pending.push((mob.join.get(), at));
        }
    }
    pending.sort_unstable_by_key(|entry| entry.0);
    for (_, at) in pending {
        let x = f64::from(game.world.random.next_int_bound(4).unwrap_or(0)) / 24.0;
        let note = ParticleOptions::simple(ParticleType::Note);
        let spread = [x, 0.0, 0.0];
        game.particles.add(&note, at, spread);
        game.lan_level_particles(None, &note, at, spread, 1.0, 0, None);
    }
}
