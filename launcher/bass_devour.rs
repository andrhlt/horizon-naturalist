fn bass_devour(game: &mut Game<'_>, target: u32, source: horizon_client::mobs::Damage)
    -> std::ops::ControlFlow<bool> {
    use horizon_client::{items::ItemStack, mobs::{Who, nav::naturalist_bass_state as bass}};
    use std::ops::ControlFlow;

    let Some(Who::Mob(attacker)) = source.attacker else { return ControlFlow::Continue(()) };
    let Some(predator) = game.mobs.iter().position(|mob| {
        mob.id == attacker && mob.kind.id == "naturalist:bass" && mob.alive()
    }) else {
        return ControlFlow::Continue(());
    };
    let Some(prey) = game.mobs.iter().position(|mob| {
        mob.id == target && mob.kind.id == "naturalist:bass" && mob.alive()
    }) else {
        return ControlFlow::Continue(());
    };
    let tier = game.mobs[predator].extra.variant;
    if predator == prey || game.mobs[prey].extra.variant >= tier {
        return ControlFlow::Continue(());
    }

    let (grew, pitch, at, left_school) = {
        let mob = &mut game.mobs[predator];
        let grew = tier == 1 && mob.random.next_float() < 0.2;
        let pitch = 1.2 + mob.random.next_float() * 0.1;
        bass::set_cooldown(&mut mob.ai.nav, 5000);
        mob.ai.target = None;
        let left_school = if grew {
            mob.extra.variant = 2;
            mob.extra.leader.take()
        } else {
            None
        };
        (grew, pitch, mob.center(), left_school)
    };
    if let Some(leader) = left_school
        && let Some(fish) = game.mobs.iter_mut().find(|fish| fish.id == leader && fish.alive())
    {
        fish.extra.followers -= 1;
    }
    if grew {
        super::naturalist_deer_variants::put_variant(game, attacker, "naturalist:bass_large");
    }

    let location = game.mobs[prey].position;
    game.mobs[prey].removed = true;
    if !grew {
        game.spawn_at_location(location, 0.0, ItemStack::new("minecraft:bone_meal", 1));
    }
    game.audio.play("minecraft:entity.parrot.eat", Some(at), 0.7, pitch);
    ControlFlow::Break(true)
}
