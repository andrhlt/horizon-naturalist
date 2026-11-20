use super::{Game, pick_view_vector};
use horizon_client::{
    items::ItemStack,
    mobs::Mob,
    particles::{ParticleOptions, ParticleType},
};
use horizon_world::level::removal::drop_item_stack_at;

pub(super) const NET: &str = "naturalist:capture_net";

pub(super) fn particles(game: &mut Game<'_>) {
    let Some(eye) = game.eye_position() else {
        return;
    };
    let view = pick_view_vector(game.yaw, game.pitch);
    let position = std::array::from_fn(|axis| eye[axis] + 1.5 * view[axis]);
    let mut options = ParticleOptions::simple(ParticleType::Smoke);
    options.mod_particle = Some("naturalist:capture_net_swing".into());
    game.particles.add(&options, position, [0.0; 3]);
    game.lan_send_particles(super::lan_particles::QueuedParticle {
        dimension: game.dim.id(),
        options,
        state: None,
        at: position,
        spread: [0.0; 3],
        speed: [0.0; 3],
        count: 1,
        randomization: 0,
    });
}

pub(super) fn vanilla_item(mob: &Mob) -> Option<&'static str> {
    match mob.kind.id.as_str() {
        "minecraft:bee" => Some("naturalist:bee"),
        "minecraft:silverfish" => Some("naturalist:silverfish"),
        "minecraft:endermite" => Some("naturalist:endermite"),
        _ => None,
    }
}

pub(super) fn has_passenger_or_vehicle(mob: &Mob) -> bool {
    mob.extra.mount.is_some() || mob.extra.rider.is_some() || mob.rider.is_some()
}

fn cod_bucket(game: &Game<'_>) -> Option<usize> {
    (0..36).chain(std::iter::once(super::shield::OFFHAND_SLOT)).find(|&slot| {
        game.stack_in(slot).is_some_and(|stack| stack.item.as_ref() == "minecraft:water_bucket")
    })
}

pub(super) fn can_catch_vanilla(game: &Game<'_>, index: usize) -> bool {
    let Some(mob) = game.mobs.get(index).filter(|mob| mob.alive()) else {
        return false;
    };
    (vanilla_item(mob).is_some() && !has_passenger_or_vehicle(mob))
        || mob.kind.id == "minecraft:bee"
        || (mob.kind.id == "minecraft:cod" && cod_bucket(game).is_some())
}

pub(super) fn apply_recipe(game: &mut Game<'_>, index: usize) -> bool {
    let mob = &game.mobs[index];
    let (output, ingredient) = match mob.kind.id.as_str() {
        "minecraft:bee" if has_passenger_or_vehicle(mob) => ("minecraft:dirt", None),
        "minecraft:cod" => {
            let Some(slot) = cod_bucket(game) else { return false };
            ("minecraft:cod_bucket", Some(slot))
        }
        _ => return false,
    };
    if !game.creative && let Some(slot) = ingredient {
        let stack = game.stack_in_mut(slot);
        if let Some(bucket) = stack {
            bucket.count -= 1;
            if bucket.count == 0 {
                *stack = None;
            }
        }
    }
    let mob = &game.mobs[index];
    let position = mob.position.components();
    let center = mob.center();
    let spread = [
        f64::from(mob.width()) * 0.5,
        f64::from(mob.height()) * 0.5,
        f64::from(mob.width()) * 0.5,
    ];
    let id = mob.id;
    if let Some(player) = game.player.as_ref() {
        game.native_play_sound_in_place("minecraft:entity.player.attack.sweep", player.position.components(), 7, 1.0, 1.0);
    }
    particles(game);
    for item in drop_item_stack_at(&mut game.world.random, position, 1) {
        game.spawn_item(ItemStack::new(output, item.count), item.pos, item.motion, 0);
    }
    let poof = ParticleOptions::simple(ParticleType::Poof);
    game.particles.send_particles(&poof, center, spread, 0.02, 15);
    game.lan_send_particles(super::lan_particles::QueuedParticle {
        dimension: game.dim.id(), options: poof, state: None, at: center,
        spread, speed: [0.02; 3], count: 15, randomization: 0,
    });
    game.native_play_sound_in_place("minecraft:entity.item.pickup", position, 7, 0.8, 1.0);
    game.break_leash(id, false, false);
    game.mobs[index].removed = true;
    game.swing_arm();
    true
}
