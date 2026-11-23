use super::{Game, pick_view_vector};
use horizon_client::{crafting::Ingredient, items::ItemStack};
use horizon_core::nbt::Tag;
use mod_api::{Host, SoundSource};

/// Java searches Inventory in index order, including armor before the off hand.
fn ingredient_slot(game: &Game<'_>, ingredient: &Ingredient) -> Option<usize> {
    (0..=40).find(|&slot| game.stack_in(slot).is_some_and(|s| game.recipe_ingredient_test(ingredient, s)))
}

pub(super) fn swing(game: &mut Game<'_>) {
    let Some(player) = game.player.as_ref() else { return };
    let at = player.position.components();
    let Some(eye) = game.eye_position() else { return };
    let look = pick_view_vector(game.yaw, game.pitch);
    let particle_at = std::array::from_fn(|i| eye[i] + look[i] * 1.5);
    let Some(level) = game.level(game.dim.id()) else { return };
    level.play_sound(game, "minecraft:entity.player.attack.sweep", at, SoundSource::Players, 1.0, 1.0);
    if let Some(key) = mod_api::__private::particle_named_key("naturalist:capture_net_swing") {
        level.send_particles(game, &mod_api::ParticleOptions::simple(key), None, particle_at, [0.0; 3], 0.0, 1);
    }
}

fn caught_effects(game: &mut Game<'_>, index: usize) {
    let mob = &game.mobs[index];
    let at = mob.position.components();
    let center = [at[0], at[1] + f64::from(mob.height()) * 0.5, at[2]];
    let spread = [f64::from(mob.width()) * 0.5, f64::from(mob.height()) * 0.5, f64::from(mob.width()) * 0.5];
    let Some(level) = game.level(game.dim.id()) else { return };
    if let Some(key) = mod_api::__private::particle_named_key("minecraft:poof") {
        level.send_particles(game, &mod_api::ParticleOptions::simple(key), None, center, spread, 0.02, 15);
    }
    level.play_sound(game, "minecraft:entity.item.pickup", at, SoundSource::Players, 0.8, 1.0);
}

/// CaughtEntityItem precedes fallback recipes. These five types save the complete mob.
fn full_item(kind: &str) -> Option<&'static str> {
    match kind {
        "minecraft:bee" => Some("naturalist:bee"),
        "minecraft:silverfish" => Some("naturalist:silverfish"),
        "minecraft:endermite" => Some("naturalist:endermite"),
        "naturalist:dragonfly" => Some("naturalist:dragonfly"),
        "naturalist:firefly" => Some("naturalist:firefly"),
        _ => None,
    }
}

fn hand_item(kind: &str) -> Option<&'static str> {
    match kind {
        "naturalist:ant" => Some("naturalist:ant"),
        "naturalist:crab" => Some("naturalist:crab"),
        "naturalist:rat" => Some("naturalist:rat"),
        "naturalist:butterfly" => Some("naturalist:butterfly"),
        "naturalist:caterpillar" => Some("naturalist:caterpillar"),
        "naturalist:desert_scorpion" => Some("naturalist:scorpion"),
        "naturalist:snail" => Some("naturalist:snail"),
        _ => None,
    }
}

pub(super) fn capture(game: &mut Game<'_>, id: i32) -> bool {
    let Some(index) = game.mobs.iter().position(|m| m.id == id as u32 && m.alive()) else { return false };
    let mob = &game.mobs[index];
    let mounted = mob.extra.mount.is_some() || mob.extra.rider.is_some() || mob.rider.is_some();
    let kind = mob.kind.id.clone();
    if kind == "naturalist:rat" && !mob.extra.tame { return false; }
    let tamed_with_owner = mob.extra.tame && mob.extra.owner.is_some();
    let crab_held = (kind == "naturalist:crab").then(|| horizon_client::mobs::finalize::item_stack(mob, 0)).flatten();
    let crab_sitting = mob.extra.sitting;
    let crab_following = game.persist.raw.get(&mob.id).and_then(|raw| raw.get("FollowingOwner")).cloned().unwrap_or(Tag::Byte(1));
    let rat_sitting = mob.extra.sitting;
    let rat_following = game.persist.raw.get(&mob.id).and_then(|raw| raw.get("FollowingOwner")).cloned().unwrap_or(Tag::Byte(1));
    let full = full_item(&kind).filter(|_| !mounted);
    if let Some(item) = full.or_else(|| hand_item(&kind)) {
        let mut saved = game.mob_tree_nbt(index);
        let mut caught = ItemStack::new(item, 1);
        if full.is_none() && let Tag::Compound(fields) = &mut saved {
            let name = fields.remove(&"CustomName".into());
            caught.data_mut().set_other("minecraft:custom_name", name);
            fields.retain(|key, value| match key.to_string_lossy().as_str() {
                "Health" | "Variant" => true,
                "Owner" => matches!(kind.as_str(), "naturalist:ant" | "naturalist:crab" | "naturalist:rat") && tamed_with_owner,
                "Age" => matches!(kind.as_str(), "naturalist:butterfly" | "naturalist:caterpillar" | "naturalist:crab" | "naturalist:rat"),
                "FollowingOwner" | "Sitting" => kind == "naturalist:rat" && tamed_with_owner,
                "Color" => kind == "naturalist:snail",
                "NoAI" | "Silent" | "NoGravity" | "Glowing" | "Invulnerable" => !matches!(value, Tag::Byte(0)),
                _ => false,
            });
            if matches!(kind.as_str(), "naturalist:ant" | "naturalist:crab" | "naturalist:rat") && tamed_with_owner {
                fields.insert("Tame".into(), Tag::Byte(1));
            }
            if kind == "naturalist:rat" && tamed_with_owner {
                fields.insert("FollowingOwner".into(), rat_following);
                fields.insert("Sitting".into(), Tag::Byte(i8::from(rat_sitting)));
            }
            if kind == "naturalist:crab" {
                if let Some(stack) = crab_held { fields.insert("HeldItem".into(), horizon_client::save::item_to_nbt(&stack)); }
                if tamed_with_owner {
                    fields.insert("FollowingOwner".into(), crab_following);
                    fields.insert("Sitting".into(), Tag::Byte(i8::from(crab_sitting)));
                }
            }
            if kind == "naturalist:snail" && let Some(Tag::Byte(color)) = fields.get(&"Color".into()) {
                fields.insert("Color".into(), Tag::Int(i32::from(*color)));
            }
        }
        caught.data_mut().set_other("minecraft:custom_data", Some(saved));
        swing(game);
        caught_effects(game, index);
        game.damage_held(1);
        if let Some(leftover) = game.add_stack_to_inventory(caught) {
            // ItemHelper.spawnItemOnEntity uses the player's drop path.
            if let Some([x, y, z]) = game.eye_position() {
                let motion = horizon_client::entities::drop_motion(&mut game.world.random, game.yaw, game.pitch, true);
                game.spawn_item(leftover, [x, y - f64::from(0.3_f32), z], motion, 40);
            }
        }
        if kind == "naturalist:ant" { crate::game::naturalist_ant_worker::release(game, id as u32); }
        game.break_leash(id as u32, false, false);
        game.mobs[index].removed = true;
        return true;
    }
    let kind = game.mobs[index].kind.id.clone();
    // Read the effective recipe table each time, so /reload and datapack overrides apply.
    for recipe in game.recipes_of_type("naturalist:net").iter() {
        if recipe.json["entity_type"].as_str() != Some(&kind) { continue }
        let ingredient = match recipe.json.get("ingredient") {
            Some(json) => {
                let Some(parsed) = Ingredient::parse(json, game.items()) else { continue };
                let Some(slot) = ingredient_slot(game, &parsed) else { continue };
                Some(slot)
            }
            None => None,
        };
        let Some(json) = recipe.json.get("result") else { continue };
        let Ok(Some(output)) = game.recipe_result(&json.to_string()) else { continue };
        if !game.creative && let Some(slot) = ingredient {
            let stack = game.stack_in_mut(slot);
            if let Some(value) = stack {
                value.count -= 1;
                if value.count == 0 { *stack = None; }
            }
        }
        swing(game);
        let at = game.mobs[index].position.components();
        for drop in horizon_world::level::removal::drop_item_stack_at(&mut game.world.random, at, output.count) {
            let mut stack = output.clone();
            stack.count = drop.count;
            game.spawn_item(stack, drop.pos, drop.motion, 0);
        }
        caught_effects(game, index);
        game.break_leash(id as u32, false, false);
        game.mobs[index].removed = true;
        return true;
    }
    false
}

/// CaughtEntityItem release preserves complete saves or constructs a fresh mob from hand traits.
pub(super) fn release(game: &mut Game<'_>, stack: &ItemStack) -> Option<mod_api::InteractionResult> {
    use mod_api::InteractionResult as Result;
    let kind = match stack.item.as_ref() {
        "naturalist:bee" => "minecraft:bee",
        "naturalist:silverfish" => "minecraft:silverfish",
        "naturalist:endermite" => "minecraft:endermite",
        "naturalist:dragonfly" => "naturalist:dragonfly",
        "naturalist:firefly" => "naturalist:firefly",
        "naturalist:ant" => "naturalist:ant",
        "naturalist:crab" => "naturalist:crab",
        "naturalist:rat" => "naturalist:rat",
        "naturalist:butterfly" => "naturalist:butterfly",
        "naturalist:caterpillar" => "naturalist:caterpillar",
        "naturalist:scorpion" => "naturalist:desert_scorpion",
        "naturalist:snail" => "naturalist:snail",
        _ => return None,
    };
    let Some(hit) = game.pick() else { return Some(Result::Pass) };
    let pos = hit.pos;
    if !game.may_interact_at(horizon_core::position::BlockPos::new(pos[0], pos[1], pos[2])) || !game.may_use_on(pos) { return Some(Result::Fail) }
    if game.remote.is_some() { return Some(Result::Success) }
    let mut fields = stack.custom_data().cloned().unwrap_or_default();
    let at = if fields.contains_key(&"id".into()) {
        let height = game.block_collision_shape(pos).bounds().map_or(0.0, |s| s.max_position().y);
        let at = [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + height, f64::from(pos[2]) + 0.5];
        fields.insert("Pos".into(), Tag::List(at.into_iter().map(|v| Tag::Double(v.to_bits())).collect()));
        fields.insert("PersistenceRequired".into(), Tag::Byte(1));
        // Java ignores a rejected spawn but still plays the release effects and consumes the item.
        let saved_age = fields
            .get(&"Age".into())
            .and_then(horizon_world::datafix::dynamic::as_number)
            .map(|number| number.int_value());
        if let Some(crate::game::commands::Target::Mob(id)) =
            game.spawn_entity_from_nbt(Tag::Compound(fields))
            && let Some(age) = saved_age
            && let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id)
        {
            // Some Naturalist species are initialized as adults during the
            // generic entity load even when the captured NBT carries Age.
            // Restore the Java-owned age after that initialization so release
            // and the subsequent world save preserve the captured value.
            mob.extra.age = age;
        }
        at
    } else {
        // A newly given item has no saved id. Use its declared type and ordinary stack configuration.
        if let Some(id) = game.spawn_mob_at(kind, pos.into(), true) {
            if let Some(mob) = game.mobs.iter_mut().find(|m| m.id == id) {
                let mut configured = stack.clone();
                configured.data_mut().set_other("minecraft:bucket_entity_data", Some(Tag::Compound(fields.clone())));
                horizon_client::entity_nbt::apply_item_components(mob, &configured);
                mob.persistent = true;
                if matches!(kind, "naturalist:ant" | "naturalist:crab" | "naturalist:rat") {
                    let saved = Tag::Compound(fields.clone());
                    let tame = saved.get("Tame").and_then(horizon_world::datafix::dynamic::as_number)
                        .is_some_and(|n| n.int_value() != 0);
                    if tame && let Some(owner) = horizon_client::entity_nbt::owner_uuid(&saved) {
                        // Naturalist pets do not inherit the Wolf template's taming health side effects.
                        mob.extra.owner = Some(owner);
                        mob.extra.tame = true;
                        if matches!(kind, "naturalist:crab" | "naturalist:rat") {
                            let sitting = saved.get("Sitting").and_then(horizon_world::datafix::dynamic::as_number).is_some_and(|n| n.int_value() != 0);
                            horizon_client::mobs::tame::restore_sitting(mob, sitting);
                        }
                    }
                }
                if kind == "naturalist:crab" && let Some(held) = fields.get(&"HeldItem".into()).and_then(horizon_client::save::item_from_nbt) {
                    horizon_client::mobs::finalize::set_item_stack(mob, 0, held);
                    horizon_client::mobs::finalize::set_drop_chance(mob, 0, 2.0);
                }
            }
            let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
            if let Tag::Compound(raw) = raw {
                // The engine preserves mod-owned fields through saves without reusing vanilla slots.
                for (key, value) in fields { raw.insert(key, value); }
                if hand_item(kind).is_some() { raw.insert("FromHand".into(), Tag::Byte(1)); }
                // CaughtEntityItem.tryAssignWorkstation runs at the clicked block position
                // immediately after spawning a Rat. It only assigns tamed workers to containers.
                if kind == "naturalist:rat"
                    && raw.get(&"Tame".into()).and_then(horizon_world::datafix::dynamic::as_number)
                        .is_some_and(|n| n.int_value() != 0)
                    && game.world.block_entities.contains_key(&(pos[0], pos[1], pos[2]))
                    && horizon_client::containers::BlockEntity::has_builtin_item_container(
                        &game.res.facts().get(game.world.block(pos[0], pos[1], pos[2])).name)
                {
                    let workstation = ((pos[0] as i64 & 0x3ffffff) << 38)
                        | ((pos[2] as i64 & 0x3ffffff) << 12)
                        | (pos[1] as i64 & 0xfff);
                    raw.insert("Workstation".into(), Tag::Long(workstation));
                    raw.insert("FollowingOwner".into(), Tag::Byte(0));
                    raw.insert("Sitting".into(), Tag::Byte(0));
                }
            }
        }
        [f64::from(pos[0]) + 0.5, f64::from(pos[1]), f64::from(pos[2]) + 0.5]
    };
    let who = game.who_context(Some(horizon_client::mobs::Who::Player), None);
    game.post_game_event("minecraft:entity_place", at, who);
    if let Some(level) = game.level(game.dim.id()) {
        let sound = match kind {
            "naturalist:butterfly" => "naturalist:entity.bird.fly",
            "naturalist:caterpillar" | "naturalist:snail" => "naturalist:entity.snail.forward",
            "naturalist:desert_scorpion" => "naturalist:entity.scorpion.ambient",
            _ => "minecraft:item.bundle.drop_contents",
        };
        level.play_sound(game, sound, [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + 0.5, f64::from(pos[2]) + 0.5], SoundSource::Neutral, 1.0, 1.0);
    }
    game.award_stat("used", &stack.item, 1);
    game.consume_held();
    Some(Result::SuccessServer)
}
