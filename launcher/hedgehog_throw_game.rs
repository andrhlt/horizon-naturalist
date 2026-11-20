use super::{Game, commands::Target, pick_view_vector};
use horizon_client::{
    items::ItemStack,
    mobs::{Damage, DamageKind, NamedDamage, PlayerView, Who, naturalist_hedgehog as hedgehog},
};
use horizon_core::{geometry::Vec3, random::RandomSource};
use horizon_world::nbt::{Compound, Tag};

pub(super) fn release_fields(stack: &ItemStack, mut fields: Compound) -> Compound {
    fields.insert("id".into(), Tag::String(hedgehog::KIND.into()));
    fields.insert("FromHand".into(), Tag::Byte(1));
    fields.insert("PersistenceRequired".into(), Tag::Byte(1));
    fields.insert("NoGravity".into(), Tag::Byte(0));
    if let Some(name) = stack.data().and_then(|data| data.custom_name.clone()) {
        fields.insert("CustomName".into(), name);
    }
    if !stack.enchantments().is_empty() {
        let tags = stack.enchantments().iter().map(|(id, level)| {
            (id.as_ref().into(), Tag::Int(*level as i32))
        }).collect();
        fields.insert("ThrowEnchantments".into(), Tag::Compound(tags));
    }
    fields
}

fn vector(v: [f64; 3]) -> Tag {
    Tag::List(v.map(|component| Tag::Double(component.to_bits())).into())
}

pub(super) fn throw(game: &mut Game<'_>, stack: &ItemStack) -> bool {
    let (Some(player), Some(eye)) = (game.player.as_ref(), game.eye_position()) else {
        return false;
    };
    let view = pick_view_vector(game.yaw, game.pitch);
    let feet = player.position.components();
    let carried_velocity = player.velocity.components();
    let mut at = std::array::from_fn(|index| feet[index] + view[index] * 0.6);
    at[1] = eye[1] - 0.3 + view[1] * 0.6;
    let motion = std::array::from_fn(|index| {
        view[index] * 1.4 + if index == 1 { 0.0 } else { carried_velocity[index] }
    });
    let mut fields = release_fields(stack, stack.custom_data().cloned().unwrap_or_default());
    fields.insert("Pos".into(), vector(at));
    fields.insert("Motion".into(), vector(motion));
    fields.insert("Rotation".into(), Tag::List(vec![Tag::Float(game.yaw.to_bits()), Tag::Float(0)]));
    let thrower = game.acting_player_uuid();
    let Some(Target::Mob(id)) = game.spawn_entity_from_nbt(Tag::Compound(fields)) else {
        return false;
    };
    let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) else {
        return false;
    };
    let pose = hedgehog::state_mut(mob);
    pose.rolling = true;
    pose.thrower = Some(thrower);

    game.native_play_sound_in_place("minecraft:entity.arrow.shoot", feet, 4, 0.5, 0.9);
    game.award_stat("used", &stack.item, 1);
    game.add_cooldown(stack.item.clone(), 20);
    game.consume_held();
    true
}

fn caught_stack(game: &mut Game<'_>, index: usize) -> ItemStack {
    let mut stack = ItemStack::new(hedgehog::KIND, 1);
    for (id, level) in &hedgehog::state(&game.mobs[index]).enchantments {
        stack.enchant(id, *level);
    }
    if let Tag::Compound(mut data) = game.mob_tree_nbt(index) {
        stack.data_mut().custom_name = data.remove(&"CustomName".into());
        data.retain(|key, nonzero| match key.to_string_lossy().as_ref() {
            "Health" | "Variant" | "Age" | "DyeColor" | "Owner" | "FollowingOwner" | "Sitting" => true,
            "NoAI" | "Silent" | "NoGravity" | "Glowing" | "Invulnerable" => !matches!(nonzero, Tag::Byte(0)),
            _ => false,
        });
        if data.contains_key(&"Owner".into()) {
            data.insert("Tame".into(), Tag::Byte(1));
        }
        stack.data_mut().set_other("minecraft:custom_data", Some(Tag::Compound(data)));
    }
    stack
}

fn return_to_player(game: &mut Game<'_>, id: u32) {
    let Some(index) = game.mobs.iter().position(|mob| mob.id == id) else { return; };
    let mob = &game.mobs[index];
    let pose = hedgehog::state(mob);
    let owner = pose.thrower.or(mob.extra.owner);
    let age = pose.return_ticks.unwrap_or(0);
    let player = game.player_view().filter(|player| Some(player.uuid) == owner)
        .map(|player| (Target::Player, player))
        .or_else(|| game.lan_guest_views().into_iter()
            .find_map(|(conn, player)| (Some(player.uuid) == owner).then_some((Target::Guest(conn), player))));
    let Some((target, side)) = player.filter(|(_, player)| player.alive && age < 200) else {
        hedgehog::stop_rolling(&mut game.mobs[index]);
        return;
    };
    let mob = &mut game.mobs[index];
    hedgehog::state_mut(mob).return_ticks = Some(age + 1);
    let center = side.position.add_vector(Vec3::new(0.0,
        (side.bounds.max_position().y - side.bounds.min_position().y - f64::from(mob.height())) * 0.5,
        0.0,
    ));
    let delta = mob.position.vector_to(center);
    let distance = delta.length();
    if distance < 0.25 {
        mob.no_gravity = false;
        let at = mob.position.components();
        let pitch = ((mob.random.next_float() - mob.random.next_float()) * 0.7 + 1.0) * 2.0;
        let stack = caught_stack(game, index);
        game.for_player(target, |game| {
            if let Some(rest) = game.give(stack) {
                game.spawn_item_own(rest, side.position.components(), 40);
            }
        });
        game.native_play_sound_in_place("minecraft:entity.item.pickup", at, 4, 0.3, pitch);
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) {
            mob.removed = true;
        }
    } else if distance < 1.5 {
        mob.velocity = Vec3::ZERO;
        mob.position = mob.position.add_vector(if distance > 0.5 { delta.normalize().scale(0.5) } else { delta });
    } else {
        mob.velocity = mob.velocity.scale(0.85).add_vector(delta.normalize().scale(0.25));
    }
}

fn thrown_contact(game: &mut Game<'_>, id: u32, victim: u32) -> bool {
    let Some(mob) = game.mobs.iter().find(|mob| mob.id == id) else { return false; };
    let position = mob.position.components();
    let motion = mob.velocity;
    let owner = mob.extra.owner.is_some();
    let amount = 2.0 + hedgehog::enchantment(mob, "minecraft:thorns") as f32;
    let source = Damage {
        kind: DamageKind::Thrown,
        attacker: owner.then_some(Who::Player),
        from: Some(position),
        attacker_species: None,
    };
    if !game.mob_hits_mob(victim, amount, source) {
        return false;
    }
    finish_contact(game, id, Some(victim), motion, source);
    true
}

fn finish_contact(game: &mut Game<'_>, id: u32, victim: Option<u32>, motion: Vec3, source: Damage) {
    let Some(index) = game.mobs.iter().position(|mob| mob.id == id) else { return; };
    let mob = &mut game.mobs[index];
    let punch = hedgehog::enchantment(mob, "minecraft:punch");
    let flame = hedgehog::enchantment(mob, "minecraft:flame") > 0;
    let unbreaking = hedgehog::enchantment(mob, "minecraft:unbreaking") as i32;
    let wears = mob.random.next_int_bound(unbreaking + 1).ok() == Some(0);
    let at = mob.center();
    if let Some(victim) = victim
        && let Some(target) = game.mobs.iter_mut().find(|mob| mob.id == victim)
    {
        if punch > 0 {
            target.knockback(f64::from(punch) * 0.6, -motion.x, -motion.z, &source);
        }
        if flame {
            target.fire_ticks = target.fire_ticks.max(100);
        }
    }

    let stack = caught_stack(game, index);
    let flecks = game.item_particle(&stack);
    game.with_level(|_, level, _| level.native_send_particles(&flecks, None, at, [0.1; 3], 0.05, 8));
    if wears {
        game.mob_hits_mob(id, 2.0, Damage {
            kind: DamageKind::Named(NamedDamage::new("naturalist:hedgehog_throw", 0)),
            attacker: victim.map(Who::Mob),
            from: None,
            attacker_species: None,
        });
    }
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) {
        if mob.alive() && hedgehog::enchantment(mob, "minecraft:loyalty") > 0 {
            hedgehog::state_mut(mob).return_ticks = Some(0);
            mob.no_gravity = true;
            mob.velocity = motion.scale(-0.5).add_vector(Vec3::new(0.0, 0.3, 0.0));
        } else {
            mob.velocity = Vec3::ZERO;
            hedgehog::stop_rolling(mob);
        }
    }
}

fn spike_players(game: &mut Game<'_>, hedgehog_id: u32, area: horizon_core::geometry::Aabb, rolling: bool) {
    let Some(mob) = game.mobs.iter().find(|mob| mob.id == hedgehog_id) else { return; };
    let owner = mob.extra.owner;
    let source = mob.position.components();
    let motion = mob.velocity;
    let thorns = hedgehog::enchantment(mob, "minecraft:thorns");
    let flame = hedgehog::enchantment(mob, "minecraft:flame") > 0;
    let eligible = |player: &PlayerView| player.alive && !player.spectator
        && owner != Some(player.uuid) && area.intersects(player.bounds);
    let mut players = Vec::new();
    if game.player_view().as_ref().is_some_and(&eligible) {
        players.push(Target::Player);
    }
    players.extend(game.lan_guest_views().into_iter()
        .filter_map(|(conn, player)| eligible(&player).then_some(Target::Guest(conn))));
    for target in players {
        let amount = if rolling { 2.0 + thorns as f32 } else { 2.0 };
        let kind = if rolling { "minecraft:thrown" } else { "minecraft:mob_attack" };
        let landed = game.for_player(target, |game| {
            let landed = game.hurt_player_from(amount, Some(source), kind, Some(Some(hedgehog_id)));
            if landed && rolling && flame
                && let Some(player) = game.player.as_mut()
            {
                player.set_remaining_fire_ticks(player.fire_ticks.max(100));
            }
            landed
        }).unwrap_or(false);
        if landed && rolling {
            finish_contact(game, hedgehog_id, None, motion, Damage::of(DamageKind::Thrown));
            break;
        }
    }
}

pub(super) fn update_visible_players(game: &mut Game<'_>) {
    let guests = game.lan_guest_views();
    if guests.is_empty() {
        for mob in game.mobs.iter_mut().filter(|mob| mob.kind.id == hedgehog::KIND) {
            hedgehog::state_mut(mob).visible_lan_player = None;
        }
        return;
    }
    let world = game.mob_world_base(&[]);
    let players = world.player.iter().chain(guests.iter().map(|(_, player)| player));
    let sightings: Vec<_> = game.mobs.iter().enumerate()
        .filter(|(_, mob)| mob.kind.id == hedgehog::KIND && !mob.extra.tame)
        .map(|(index, mob)| {
            (index, players.clone().any(|player| hedgehog::visible_player(mob, &world, player)))
        })
        .collect();
    for (index, visible) in sightings {
        hedgehog::state_mut(&mut game.mobs[index]).visible_lan_player = Some(visible);
    }
}

pub(super) fn tick(game: &mut Game<'_>) {
    if game.remote.is_some() {
        return;
    }
    let active: Vec<_> = game.mobs.iter().filter(|mob| {
        mob.kind.id == hedgehog::KIND && mob.alive() && !mob.removed
            && (hedgehog::state(mob).rolling || hedgehog::state(mob).hiding && mob.age % 10 == 0)
    }).map(|mob| mob.id).collect();

    for id in active {
        let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) else { continue; };
        let rolling = hedgehog::state(mob).rolling;
        if rolling && hedgehog::enchantment(mob, "minecraft:flame") > 0 {
            mob.fire_ticks = 2;
        }
        if hedgehog::state(mob).return_ticks.is_some() {
            return_to_player(game, id);
            continue;
        }
        let at = mob.position.components();
        let area = mob.bounds().inflate(Vec3::new(0.2, 0.2, 0.2));
        let victims: Vec<_> = game.mobs.iter().filter(|victim| {
            victim.kind.id != hedgehog::KIND && victim.alive() && !victim.removed && area.intersects(victim.bounds())
        }).map(|victim| victim.id).collect();
        let mut landed = false;
        for victim in victims {
            if rolling {
                if thrown_contact(game, id, victim) {
                    landed = true;
                    break;
                }
            } else {
                game.mob_hits_mob(victim, 2.0, Damage {
                    kind: DamageKind::MobAttack, attacker: Some(Who::Mob(id)),
                    from: Some(at), attacker_species: None,
                });
            }
        }
        if !landed {
            spike_players(game, id, area, rolling);
        }
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) {
            let pose = hedgehog::state(mob);
            if pose.rolling && pose.return_ticks.is_none() && (mob.on_ground || mob.fluids.in_water) {
                hedgehog::stop_rolling(mob);
            }
        }
    }
}
