fn knapsack_capture(game: &mut Game<'_>, entity: i32, stack: Option<&ItemStack>, swing: &mut Option<bool>) {
    use super::naturalist_knapsack as sack;
    use horizon_client::particles::{ParticleOptions, ParticleType};

    let Some(source) = stack.filter(|stack| stack.item.as_ref() == sack::ITEM && !sack::filled(stack)) else { return };
    if swing.is_some() {
        return;
    }
    let Some(index) = game.mobs.iter().position(|mob| mob.id == entity as u32 && sack::capturable(mob)) else { return };
    let mob = &game.mobs[index];
    let id = mob.id;
    let at = mob.position.components();
    let center = mob.center();
    let spread = [f64::from(mob.width()) * 0.5, f64::from(mob.height()) * 0.5, f64::from(mob.width()) * 0.5];
    let mut captured = ItemStack::new(sack::ITEM, 1);
    captured.data_mut().set_other("minecraft:custom_data", Some(game.mob_tree_nbt(index)));

    let player = game.player.as_ref().map(|player| player.position.components());
    if let Some(at) = player {
        game.native_play_sound_in_place("minecraft:entity.player.attack.sweep", at, 7, 1.0, 1.0);
    }
    super::naturalist_capture_net_swing::particles(game);
    let options = ParticleOptions::simple(ParticleType::Poof);
    game.particles.send_particles(&options, center, spread, 0.02, 15);
    game.lan_send_particles(super::lan_particles::QueuedParticle {
        dimension: game.dim.id(), options, state: None, at: center,
        spread, speed: [0.02; 3], count: 15, randomization: 0,
    });
    game.native_play_sound_in_place("minecraft:entity.item.pickup", at, 7, 0.8, 1.0);
    game.break_leash(id, false, false);
    game.native_eject_passengers(id);
    game.mobs[index].removed = true;
    game.swing_arm();
    if let Some(at) = player {
        game.native_play_sound_in_place("naturalist:item.knapsack.pickup", at, 6, 0.6, 1.0);
    }
    sack::replace_one(game, source, captured);
    *swing = Some(false);
}
