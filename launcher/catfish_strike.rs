fn catfish_strike(game: &mut Game<'_>, target: u32, source: horizon_client::mobs::Damage) -> std::ops::ControlFlow<bool> {
    use horizon_client::{items::ItemStack, mobs::{Who, nav::naturalist_catfish_state as catfish}};
    use horizon_core::random::RandomSource;
    use std::ops::ControlFlow;

    let Some(Who::Mob(attacker)) = source.attacker else { return ControlFlow::Continue(()) };
    let Some(hunter) = game.mobs.iter().position(|mob| {
        mob.id == attacker && mob.kind.id == "naturalist:catfish" && mob.alive() && !mob.removed
    }) else { return ControlFlow::Continue(()) };
    let Some(victim) = game.mobs.iter().position(|mob| {
        mob.id == target && mob.kind.id == "naturalist:bass" && mob.alive() && !mob.removed
    }) else { return ControlFlow::Continue(()) };
    let predator = &mut game.mobs[hunter];
    match catfish::prey(&predator.ai.nav) {
        None => {
            catfish::set_prey(&mut predator.ai.nav, Some((target, catfish::EAT_TICKS)));
            catfish::set_cooldown(&mut predator.ai.nav, catfish::COOLDOWN_TICKS);
            predator.ai.target = None;
        }
        Some((queued, 0)) if queued == target => {
            catfish::set_prey(&mut predator.ai.nav, None);
            let pitch = 1.2 + predator.random.next_float() * 0.1;
            let at = predator.position.components();
            let silent = predator.silent;
            let options = game.item_particle(&ItemStack::new("naturalist:bass", 1));
            let prey = &mut game.mobs[victim];
            let location = prey.position;
            let size = prey.bounds().size();
            for _ in 0..16 {
                let x = size.x * (2.0 * prey.random.next_double() - 1.0);
                let y = size.y * prey.random.next_double();
                let z = size.z * (2.0 * prey.random.next_double() - 1.0);
                game.particles.send_particles(&options, [location.x + x, location.y + y, location.z + z], [0.0; 3], 0.05, 1);
            }
            prey.removed = true;
            game.spawn_at_location(location, 0.0, ItemStack::new("minecraft:bone_meal", 1));
            if !silent {
                game.audio.play("minecraft:entity.parrot.eat", Some(at), 0.7, pitch);
            }
        }
        _ => return ControlFlow::Continue(()),
    }
    ControlFlow::Break(true)
}
