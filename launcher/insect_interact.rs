fn insect_interact(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use super::naturalist_capture_net_swing::{self as net, NET};
    use horizon_client::particles::{ParticleOptions, ParticleType};
    use horizon_world::nbt::Tag;
    use std::ops::ControlFlow;
    let Some(index) = game.mob_interaction_target(target) else { return ControlFlow::Continue(()) };
    let mob = &game.mobs[index];
    if !mob.alive() {
        return ControlFlow::Continue(());
    }
    let held = game.held().cloned();
    let item = held.as_ref().map_or("", |stack| stack.item.as_ref());
    if item == NET && net::apply_recipe(game, index) {
        return ControlFlow::Break(Ok(true));
    }
    let mob = &game.mobs[index];
    let (caught_item, full_capture) = match mob.kind.id.as_str() {
        "naturalist:hedgehog" if mob.extra.tame => ("naturalist:hedgehog", false),
        "naturalist:ant" => ("naturalist:ant", false),
        "naturalist:snail" => ("naturalist:snail", false),
        "naturalist:butterfly" => ("naturalist:butterfly", false),
        "naturalist:caterpillar" => ("naturalist:caterpillar", false),
        "naturalist:desert_scorpion" => ("naturalist:scorpion", false),
        "naturalist:dragonfly" => ("naturalist:dragonfly", true),
        "naturalist:firefly" => ("naturalist:firefly", true),
        "naturalist:rat" if mob.extra.tame => ("naturalist:rat", false),
        "naturalist:crab" => ("naturalist:crab", false),
        _ => match net::vanilla_item(mob) {
            Some(item) => (item, true),
            None => return ControlFlow::Continue(()),
        },
    };
    let id = mob.id;
    if item == NET {
        let center = mob.center();
        let vanilla = net::vanilla_item(mob).is_some();
        let origin = mob.position.components();
        let radius = f64::from(mob.width()) * 0.5;
        let spread = [radius, f64::from(mob.height()) * 0.5, radius];
        let mut captured = ItemStack::new(caught_item, 1);
        if full_capture && net::has_passenger_or_vehicle(mob) {
            return ControlFlow::Continue(());
        }
        if caught_item == "naturalist:hedgehog" {
            for (id, level) in &horizon_client::mobs::naturalist_hedgehog::state(mob).enchantments {
                captured.enchant(id, *level);
            }
        }
        let owned = mob.extra.tame && mob.extra.owner.is_some();
        let mut saved = game.mob_tree_nbt(index);
        if !full_capture && let Tag::Compound(fields) = &mut saved {
            let name = fields.remove(&"CustomName".into());
            captured.data_mut().set_other("minecraft:custom_name", name);
            let crab = caught_item == "naturalist:crab";
            let rat = caught_item == "naturalist:rat";
            let ant = caught_item == "naturalist:ant";
            if crab {
                if let Some(weapon) = fields.get(&"equipment".into()).and_then(|data| data.get("mainhand")).cloned() {
                    fields.insert("HeldItem".into(), weapon);
                }
                if owned {
                    fields.insert("Tame".into(), Tag::Byte(1));
                } else {
                    for key in ["Owner", "FollowingOwner", "Sitting"] {
                        fields.remove(&key.into());
                    }
                }
            }
            if (ant || rat || caught_item == "naturalist:hedgehog") && owned {
                fields.insert("Tame".into(), Tag::Byte(1));
            }
            fields.retain(|key, value| match key.to_string_lossy().as_str() {
                "Health" | "Variant" => true,
                "Age" => matches!(caught_item, "naturalist:hedgehog" | "naturalist:butterfly" | "naturalist:caterpillar" | "naturalist:crab" | "naturalist:rat"),
                "Owner" | "Tame" => crab || caught_item == "naturalist:hedgehog" || ((ant || rat) && owned),
                "FollowingOwner" | "Sitting" => (rat && owned) || crab || caught_item == "naturalist:hedgehog",
                "HeldItem" => crab,
                "DyeColor" => caught_item == "naturalist:hedgehog",
                "Color" => caught_item == "naturalist:snail",
                "NoAI" | "Silent" | "NoGravity" | "Glowing" | "Invulnerable" => {
                    !matches!(value, Tag::Byte(0))
                }
                _ => false,
            });
        }
        if caught_item == "naturalist:snail"
            && let Tag::Compound(fields) = &mut saved
            && let Some(Tag::Byte(color)) = fields.get(&"Color".into())
        {
            fields.insert("Color".into(), Tag::Int(i32::from(*color)));
        }
        captured.data_mut().set_other("minecraft:custom_data", Some(saved));
        if let Some(player) = game.player.as_ref() {
            let at = [player.position.x, player.position.y, player.position.z];
            if vanilla {
                game.native_play_sound_in_place("minecraft:entity.player.attack.sweep", at, 7, 1.0, 1.0);
            } else {
                game.audio.play("minecraft:entity.player.attack.sweep", Some(at), 1.0, 1.0);
            }
        }
        super::naturalist_capture_net_swing::particles(game);
        let options = ParticleOptions::simple(ParticleType::Poof);
        game.particles.send_particles(&options, center, spread, 0.02, 15);
        game.lan_send_particles(super::lan_particles::QueuedParticle {
            dimension: game.dim.id(), options, state: None, at: center,
            spread, speed: [0.02; 3], count: 15, randomization: 0,
        });
        if vanilla {
            game.native_play_sound_in_place("minecraft:entity.item.pickup", origin, 7, 0.8, 1.0);
        } else if !matches!(caught_item, "naturalist:ant" | "naturalist:crab" | "naturalist:hedgehog" | "naturalist:rat") {
            game.audio.play("minecraft:entity.item.pickup", Some(center), 0.8, 1.0);
        }
        game.damage_held(1);
        if let Some(stack) = game.give(captured) {
            if !full_capture {
                game.trigger_item("minecraft:filled_bucket", &stack);
            }
            if vanilla {
                if let Some([x, y, z]) = game.eye_position() {
                    let velocity = horizon_client::entities::drop_motion(
                        &mut game.world.random, game.yaw, game.pitch, true,
                    );
                    game.spawn_item(stack, [x, y - f64::from(0.3_f32), z], velocity, 40);
                }
            } else if matches!(caught_item, "naturalist:ant" | "naturalist:crab" | "naturalist:hedgehog" | "naturalist:rat") {
                let at = game.player.as_ref().map_or(center, |player| player.position.components());
                game.spawn_item_own(stack, at, 0);
                game.audio.play("minecraft:entity.item.pickup", Some(at), 0.3, 1.0);
            } else {
                game.spawn_item_own(stack, center, 40);
            }
        }
        game.break_leash(id, false, false);
        game.mobs[index].removed = true;
        game.swing_arm();
        return ControlFlow::Break(Ok(true));
    }
    if caught_item == "naturalist:ant" {
        if item == "minecraft:sugar" && mob.health < mob.attrs.max_health {
            let at = mob.center();
            game.mobs[index].heal(1.0);
            game.native_play_sound_in_place("minecraft:entity.generic.eat", at, 4, 1.0, 1.0);
            game.consume_held();
            game.swing_arm();
            return ControlFlow::Break(Ok(true));
        }
        if matches!(item, "minecraft:name_tag" | "minecraft:lead")
            || (item.is_empty() && (mob.extra.leash.is_some() || mob.extra.leash_pending.is_some()))
        {
            return ControlFlow::Continue(());
        }
        return ControlFlow::Break(Ok(false));
    }
    if full_capture || matches!(caught_item, "naturalist:scorpion" | "naturalist:snail" | "naturalist:crab" | "naturalist:hedgehog" | "naturalist:rat") {
        return ControlFlow::Continue(());
    }
    let butterfly = caught_item == "naturalist:butterfly";
    let flower = game.items().item_tags.get("minecraft:flowers").is_some_and(|tag| tag.contains(item));
    if flower && (butterfly || mob.is_baby()) {
        let player = game.acting_player_uuid();
        if feed_animal(&mut game.mobs[index], &mut game.world.random, Some(player)).is_some() {
            if game.mobs[index].extra.in_love > 0 {
                let kind = game.mobs[index].kind.id.clone();
                game.love_cause.insert(id, game.lan_acting);
                game.native_entity_enter_love_mode(id, &kind, game.lan_acting_uuid);
                game.mob_event_particles(id, 18);
            }
            if let Some(held) = &held {
                let entity = game.mob_entity(&game.mobs[index]);
                game.trigger_item_entity("minecraft:player_interacted_with_entity", held, &entity);
            }
            game.consume_held();
            game.swing_arm();
        }
        return ControlFlow::Break(Ok(true));
    }
    if matches!(item, "minecraft:bucket" | "minecraft:wheat" | "minecraft:cookie"
            | "minecraft:wheat_seeds" | "minecraft:melon_seeds" | "minecraft:pumpkin_seeds"
            | "minecraft:beetroot_seeds" | "minecraft:torchflower_seeds" | "minecraft:pitcher_pod") {
        return ControlFlow::Break(Ok(false));
    }
    ControlFlow::Continue(())
}
