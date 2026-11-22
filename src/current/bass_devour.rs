fn bass_devour(game: &mut Game<'_>, target: u32, source: Damage) -> std::ops::ControlFlow<bool> {
    use horizon_client::items::ItemStack;
    use horizon_client::mobs::Who;
    use horizon_core::random::RandomSource;
    use mod_api::{Holder, Host};
    use std::ops::ControlFlow;

    let Some(Who::Mob(attacker)) = source.attacker else {
        return ControlFlow::Continue(());
    };
    let Some(predator) = game
        .mobs
        .iter()
        .position(|m| m.id == attacker && m.kind.id == "naturalist:bass" && m.alive())
    else {
        return ControlFlow::Continue(());
    };
    let Some(prey) = game
        .mobs
        .iter()
        .position(|m| m.id == target && m.kind.id == "naturalist:bass" && m.alive())
    else {
        return ControlFlow::Continue(());
    };
    if predator == prey {
        return ControlFlow::Continue(());
    }
    let Some(attacker_ref) = game.entity(attacker) else {
        return ControlFlow::Continue(());
    };
    let Some(prey_ref) = game.entity(target) else {
        return ControlFlow::Continue(());
    };
    let Some(attacker_data) = attacker_ref.nbt(game) else {
        return ControlFlow::Continue(());
    };
    let Some(prey_data) = prey_ref.nbt(game) else {
        return ControlFlow::Continue(());
    };
    let attacker_tier = naturalist::bass_hunt::tier(&attacker_data);
    let prey_tier = naturalist::bass_hunt::tier(&prey_data);
    if prey_tier >= attacker_tier {
        return ControlFlow::Continue(());
    }
    let cooldown = naturalist::meal_cooldown(
        game.creatures.deferred_hits,
        game.creatures.ticked.contains(&attacker),
    );
    if naturalist::set(game, Holder::Entity(attacker), cooldown).is_err() {
        return ControlFlow::Continue(());
    }

    let (grew, pitch, leader) = {
        let mob = &mut game.mobs[predator];
        let grew = attacker_tier == 1 && mob.random.next_float() < 0.2;
        let pitch = 1.2 + mob.random.next_float() * 0.1;
        mob.ai.target = None;
        let leader = if grew {
            mob.extra.variant = 2;
            mob.extra.leader.take()
        } else {
            None
        };
        (grew, pitch, leader)
    };
    if let Some(leader) = leader
        && let Some(fish) = game.mobs.iter_mut().find(|m| m.id == leader && m.alive())
    {
        fish.extra.followers -= 1;
    }
    if grew {
        let raw = game
            .persist
            .raw
            .entry(attacker)
            .or_insert_with(|| horizon_core::nbt::Tag::Compound(Default::default()));
        if let horizon_core::nbt::Tag::Compound(data) = raw {
            data.insert(
                "Variant".into(),
                horizon_core::nbt::Tag::String("naturalist:bass_large".into()),
            );
        }
    }
    let level = prey_ref.level(game);
    let (location, particles) = {
        let fish = &mut game.mobs[prey];
        let position = fish.position;
        let size = fish.bounds().size();
        let particles: Vec<[f64; 3]> = (0..16)
            .map(|_| {
                [
                    position.x + (fish.random.next_double() - 0.5) * size.x,
                    position.y + fish.random.next_double() * size.y,
                    position.z + (fish.random.next_double() - 0.5) * size.z,
                ]
            })
            .collect();
        (position, particles)
    };
    if let Some(level) = level {
        naturalist::bass_fx::meal(game, level, &particles);
    }
    game.mobs[prey].removed = true;
    if !grew {
        game.spawn_at_location(location, 0.0, ItemStack::new("minecraft:bone_meal", 1));
    }
    attacker_ref.play_sound(game, "minecraft:entity.parrot.eat", 0.7, pitch);
    ControlFlow::Break(true)
}
