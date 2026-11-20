fn mole_trails(game: &mut Game<'_>) {
    use horizon_client::mobs::goals::naturalist_mole_goals::{self as mole, MOLE, TRAIL};
    use horizon_core::geometry::Direction;

    use horizon_client::{mobs::Who, particles::{ParticleOptions, ParticleType}};
    let mut pending = Vec::new();
    let mut digs = Vec::new();
    for mob in &mut game.mobs {
        if mob.kind.id == MOLE {
            if let Some(at) = mole::state_mut(&mut mob.ai.nav).pending_mound.take() {
                pending.push((at, mob.is_baby()));
            }
        } else if mob.kind.id == TRAIL && mob.age >= 50 {
            mob.removed = true;
        } else if mob.kind.id == "minecraft:wolf"
            && let Some(dig) = mole::state_mut(&mut mob.ai.nav).wolf_dig.take()
        {
            digs.push((mob.id, dig));
        }
    }
    for (wolf, dig) in digs {
        let Some(animal) = game
            .mobs
            .iter_mut()
            .find(|mob| mob.id == dig.mole && mob.kind.id == MOLE && mole::can_be_dug_out(mob))
        else {
            continue;
        };
        let at = animal.position.components();
        if dig.elapsed == 49 {
            mole::pop_out(animal, Who::Mob(wolf));
        } else {
            mole::hold_for_digging(animal);
        }
        if dig.elapsed.is_multiple_of(5) {
            let block = at.map(|value| value.floor() as i32);
            if let Some((sprite, tint)) = game.particle_sprite(dig.ground, block) {
                let options = ParticleOptions::block(ParticleType::Block, sprite, tint);
                game.native_send_particles_in_place(
                    &options, Some(dig.ground), [at[0], at[1] + 0.1, at[2]],
                    [0.2, 0.05, 0.2], 0.15, 12,
                );
            }
        }
        if let Some(random_pitch) = dig.hit_pitch {
            let sound = &game.res.facts().get(dig.ground).sounds;
            let (hit, pitch) = (sound.hit.clone(), sound.pitch * random_pitch);
            game.native_play_sound_in_place(&hit, at, 6, 0.9, pitch);
        }
    }
    for ([x, y, z], small) in pending {
        let position = [f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5];
        let world = game.mob_world_base(&[]);
        if !world
            .fluid_tables
            .sturdy(world.world.block(x, y - 1, z), Direction::Up)
            || !world.block_collision_boxes(x, y, z).is_empty()
            || game.mobs.iter().any(|mob| {
                mob.kind.id == TRAIL && !mob.removed && mob.position.components() == position
            })
        {
            continue;
        }
        if let Some(id) = game.spawn_mob_raw(TRAIL, position, false)
            && let Some(trail) = game.mobs.iter_mut().find(|mob| mob.id == id)
        {
            trail.no_ai = true;
            trail.no_gravity = true;
            trail.extra.variant = u8::from(small);
        }
    }
}
