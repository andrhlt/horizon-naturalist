use super::Game;
use horizon_core::nbt::Tag;
use horizon_core::{geometry::Vec3, random::RandomSource};
use mod_api::{Holder, Host, SoundSource};
use naturalist::ant_worker::{Food, STATE};

pub(super) fn prepare(game: &mut Game<'_>, id: u32) {
    let Ok(shared) = STATE.get(game, Holder::Entity(id)) else {
        return;
    };
    let Ok(mut state) = shared.lock() else { return };
    let Some(ant) = game.mobs.iter().find(|m| m.id == id && m.alive()) else {
        return;
    };
    let meal = match game
        .persist
        .raw
        .get(&id)
        .and_then(|raw| raw.get("CarriedFood"))
    {
        Some(Tag::IntArray(words)) => <[i32; 4]>::try_from(words.as_slice()).ok(),
        _ => None,
    };
    state.carried = meal.is_some_and(|uuid| {
        game.dropped
            .iter()
            .any(|item| item.stack.count > 0 && game.persist.uuids.get(&item.id) == Some(&uuid))
    });
    if meal.is_some()
        && !state.carried
        && let Some(Tag::Compound(raw)) = game.persist.raw.get_mut(&id)
    {
        raw.remove(&"CarriedFood".into());
    }
    state.cooldown = game
        .persist
        .raw
        .get(&id)
        .and_then(|raw| raw.get("HillCooldown"))
        .and_then(horizon_world::datafix::dynamic::as_number)
        .map(|n| n.int_value())
        .unwrap_or(0);
    let area = ant.bounds().inflate(Vec3::new(8.0, 4.0, 8.0));
    let items = game.res.items();
    state.food = game
        .dropped
        .iter()
        .filter(|drop| {
            drop.pickup_delay == 0
                && drop.stack.count > 0
                && game
                    .persist
                    .raw
                    .get(&drop.id)
                    .and_then(|raw| raw.get("Ant"))
                    .is_none()
                && state
                    .pursuing
                    .map_or_else(|| area.intersects(drop.bounds()), |id| drop.id == id)
                && horizon_client::item_sync::food(
                    &drop.stack,
                    items.get(&drop.stack.item).and_then(|info| info.food),
                )
                .is_some()
        })
        .min_by(|a, b| {
            ant.distance_sqr(a.position)
                .total_cmp(&ant.distance_sqr(b.position))
        })
        .map(|drop| Food {
            id: drop.id,
            at: drop.position.components(),
        });
}

pub(super) fn finish(game: &mut Game<'_>, id: u32) {
    use horizon_core::random::RandomSource;
    // A mob that ticked and disappeared was destructively removed. Chunk
    // unloading does not send this tick-post and keeps the item for reload.
    if !game.mobs.iter().any(|ant| ant.id == id && !ant.removed) {
        release(game, id);
        return;
    }
    let Ok(Some(shared)) = STATE.get_existing(game, Holder::Entity(id)) else {
        return;
    };
    let (food, room, enter) = shared
        .lock()
        .map(|mut state| (state.pick_up.take(), state.store.take(), state.enter.take()))
        .unwrap_or((None, None, None));
    if let Some(room) = room {
        store(game, id, room);
    }
    if let Some(room) = enter {
        enter_hill(game, id, room);
    }
    if let Some(Tag::Compound(fields)) = game.persist.raw.get_mut(&id) {
        let ticks = fields
            .get(&"HillCooldown".into())
            .and_then(horizon_world::datafix::dynamic::as_number)
            .map(|n| n.int_value())
            .unwrap_or(0);
        if ticks > 0 {
            fields.insert("HillCooldown".into(), Tag::Int(ticks - 1));
        }
    }
    let Some(food) = food else { return };
    let Some(ant) = game
        .mobs
        .iter()
        .find(|m| m.id == id && m.alive() && !m.removed)
    else {
        return;
    };
    if game
        .persist
        .raw
        .get(&id)
        .and_then(|raw| raw.get("CarriedFood"))
        .is_some()
    {
        return;
    }
    let Some(index) = game
        .dropped
        .iter()
        .position(|drop| drop.id == food && drop.pickup_delay == 0)
    else {
        return;
    };
    if ant.distance_sqr(game.dropped[index].position) >= 1.0 {
        return;
    }
    let [x, y, z] = ant.position.components();
    let at = [x, y + f64::from(ant.height()) + 0.05, z];
    let mut stack = game.dropped[index].stack.clone();
    stack.count = 1;
    game.spawn_item(stack, at, [0.0; 3], 0);
    let Some(carried) = game.just_spawned_item() else {
        return;
    };
    carried.no_gravity = true;
    carried.age = i32::from(i16::MIN);
    let carried_id = carried.id;
    let item_uuid = game.uuid_for(carried_id);
    let ant_uuid = game.mob_uuid(id);
    game.persist.raw.insert(
        carried_id,
        Tag::compound([("Ant", Tag::IntArray(ant_uuid.to_vec()))]),
    );
    let raw = game
        .persist
        .raw
        .entry(id)
        .or_insert_with(|| Tag::Compound(Default::default()));
    if let Tag::Compound(fields) = raw {
        fields.insert("CarriedFood".into(), Tag::IntArray(item_uuid.to_vec()));
    }
    if game.dropped[index].stack.count == 1 {
        game.remove_dropped(index);
    } else {
        game.dropped[index].stack.count -= 1;
    }
    if let Some(ant) = game.mobs.iter_mut().find(|m| m.id == id) {
        let pitch = ((ant.random.next_float() - ant.random.next_float()) * 0.7 + 1.0) * 2.0;
        if let Some(level) = game.level(game.dim.id()) {
            level.play_sound(
                game,
                "minecraft:entity.item.pickup",
                [x, y, z],
                SoundSource::Neutral,
                0.2,
                pitch,
            );
        }
    }
}

/// The engine stores a carried-food object as an item entity; its raw Ant UUID
/// and saved id identify the Naturalist subtype without conflating plain drops.
pub(super) fn follow(game: &mut Game<'_>, id: u32) {
    let Some(raw) = game.persist.raw.get(&id) else {
        return;
    };
    let carried = raw.get("Ant").is_some()
        || matches!(raw.get("id"), Some(Tag::String(kind)) if kind.to_string_lossy() == "naturalist:carried_food");
    if !carried {
        return;
    }
    let owner = match raw.get("Ant") {
        Some(Tag::IntArray(value)) => <[i32; 4]>::try_from(value.as_slice()).ok(),
        _ => None,
    };
    let Some(index) = game.dropped.iter().position(|item| item.id == id) else {
        return;
    };
    let food_uuid = game.persist.uuids.get(&id).copied();
    let ant = game.mobs.iter().find(|m| {
        m.kind.id == "naturalist:ant"
            && m.alive()
            && !m.removed
            && owner.is_some_and(|owner| game.persist.uuids.get(&m.id) == Some(&owner))
    });
    if let Some(ant) = ant {
        let linked = match game
            .persist
            .raw
            .get(&ant.id)
            .and_then(|raw| raw.get("CarriedFood"))
        {
            Some(Tag::IntArray(value)) => <[i32; 4]>::try_from(value.as_slice()).ok() == food_uuid,
            _ => false,
        };
        if linked {
            let at = ant
                .position
                .add_vector(Vec3::new(0.0, f64::from(ant.height()) + 0.05, 0.0));
            let item = &mut game.dropped[index];
            item.previous = item.position;
            item.position = at;
            item.velocity = Vec3::ZERO;
            item.age = i32::from(i16::MIN);
            item.no_gravity = true;
            if let Some(Tag::Compound(fields)) = game.persist.raw.get_mut(&id) {
                fields.remove(&"NaturalistUnresolvedSince".into());
            }
            return;
        }
    }
    let now = game.world.game_time;
    let raw = game
        .persist
        .raw
        .entry(id)
        .or_insert_with(|| Tag::Compound(Default::default()));
    let since = if let Tag::Compound(fields) = raw {
        match fields
            .entry("NaturalistUnresolvedSince".into())
            .or_insert(Tag::Long(now))
        {
            Tag::Long(first) => *first,
            _ => now,
        }
    } else {
        now
    };
    // An alive but unlinked ant releases immediately; an absent ant has Java's 60-tick grace.
    if ant.is_some() || now.saturating_sub(since) > 60 {
        let item = game.remove_dropped(index);
        game.spawn_item_own(item.stack, item.position.components(), 10);
    }
}

fn store(game: &mut Game<'_>, id: u32, room: [i32; 3]) {
    let Some(ant) = game
        .mobs
        .iter()
        .find(|m| m.id == id && m.alive() && !m.removed)
    else {
        return;
    };
    let owner = ant.extra.owner;
    let Some(row) = game
        .pack
        .assets
        .registry
        .state(game.world.block(room[0], room[1], room[2]))
    else {
        return;
    };
    if row.name != "naturalist:ant_hill"
        || row.properties.get("open").map(String::as_str) != Some("true")
    {
        return;
    }
    let Some(horizon_client::chunk::block_entity::BlockEntity::Mod(hill)) =
        game.world.block_entities.get(&(room[0], room[1], room[2]))
    else {
        return;
    };
    let hill_owner = match hill.nbt.get(&"Owner".into()) {
        Some(Tag::IntArray(id)) => <[i32; 4]>::try_from(id.as_slice()).ok(),
        _ => None,
    };
    if hill_owner != owner {
        return;
    }
    let Some(Tag::IntArray(uuid)) = game
        .persist
        .raw
        .get(&id)
        .and_then(|raw| raw.get("CarriedFood"))
    else {
        return;
    };
    let Ok(uuid) = <[i32; 4]>::try_from(uuid.as_slice()) else {
        return;
    };
    let Some(index) = game
        .dropped
        .iter()
        .position(|item| game.persist.uuids.get(&item.id) == Some(&uuid))
    else {
        return;
    };
    let item = game.dropped[index].stack.clone();
    let limit = game.stack_max_size(&item);
    let stored = game
        .native_with_mod_block_entity(room, |hill| {
            let list = hill
                .nbt
                .entry("Storage".into())
                .or_insert_with(|| Tag::List(Vec::new()));
            let Tag::List(stacks) = list else {
                return false;
            };
            for entry in stacks.iter_mut() {
                let Some(mut stack) = horizon_client::save::item_from_nbt(entry) else {
                    continue;
                };
                if stack.count < limit && stack.same_item_same_components(&item) {
                    stack.count += 1;
                    *entry = horizon_client::save::item_to_nbt(&stack);
                    return true;
                }
            }
            if stacks.len() >= 9 {
                return false;
            }
            stacks.push(horizon_client::save::item_to_nbt(&item));
            true
        })
        .unwrap_or(false);
    game.remove_dropped(index);
    if let Some(Tag::Compound(fields)) = game.persist.raw.get_mut(&id) {
        fields.remove(&"CarriedFood".into());
    }
    if !stored {
        game.pop_resource(room, item);
    }
    let pitch = game
        .mobs
        .iter_mut()
        .find(|m| m.id == id)
        .map(|ant| 0.9 + ant.random.next_float() * 0.2)
        .unwrap_or(1.0);
    if let Some(level) = game.level(game.dim.id()) {
        let at = room.map(|axis| f64::from(axis) + 0.5);
        level.play_sound(
            game,
            "minecraft:block.beehive.exit",
            at,
            SoundSource::Blocks,
            0.9,
            pitch,
        );
        if let Some(key) = mod_api::__private::particle_named_key("minecraft:poof") {
            level.send_particles(
                game,
                &mod_api::ParticleOptions::simple(key),
                None,
                at,
                [0.25; 3],
                0.0,
                8,
            );
        }
    }
}

/// A destructive removal, including a Capture Net, leaves the food behind.
pub(super) fn release(game: &mut Game<'_>, id: u32) {
    let uuid = match game
        .persist
        .raw
        .get(&id)
        .and_then(|raw| raw.get("CarriedFood"))
    {
        Some(Tag::IntArray(words)) => <[i32; 4]>::try_from(words.as_slice()).ok(),
        _ => None,
    };
    if let Some(Tag::Compound(fields)) = game.persist.raw.get_mut(&id) {
        fields.remove(&"CarriedFood".into());
    }
    if let Some(uuid) = uuid
        && let Some(index) = game
            .dropped
            .iter()
            .position(|item| game.persist.uuids.get(&item.id) == Some(&uuid))
    {
        let item = game.remove_dropped(index);
        game.spawn_item_own(item.stack, item.position.components(), 10);
    }
}

fn enter_hill(game: &mut Game<'_>, id: u32, room: [i32; 3]) {
    let Some(ant) = game
        .mobs
        .iter()
        .find(|m| m.id == id && m.alive() && !m.removed)
    else {
        return;
    };
    let owner = ant.extra.owner;
    let Some(row) = game
        .pack
        .assets
        .registry
        .state(game.world.block(room[0], room[1], room[2]))
    else {
        return;
    };
    let workers = row
        .properties
        .get("workers")
        .and_then(|v| v.parse::<u8>().ok());
    let open = row.name == "naturalist:ant_hill"
        && row.properties.get("open").map(String::as_str) == Some("true");
    let hill_owner = match game.world.block_entities.get(&(room[0], room[1], room[2])) {
        Some(horizon_client::chunk::block_entity::BlockEntity::Mod(hill)) => {
            match hill.nbt.get(&"Owner".into()) {
                Some(Tag::IntArray(id)) => <[i32; 4]>::try_from(id.as_slice()).ok(),
                _ => None,
            }
        }
        _ => return,
    };
    let entered = open
        && hill_owner == owner
        && workers.is_some_and(|w| w < 3)
        && game.set_block_property_with(room, "workers", &(workers.unwrap() + 1).to_string(), 3);
    if entered {
        let pitch = if let Some(ant) = game.mobs.iter_mut().find(|m| m.id == id) {
            ant.removed = true;
            0.9 + ant.random.next_float() * 0.2
        } else {
            1.0
        };
        if let Some(level) = game.level(game.dim.id()) {
            let at = room.map(|axis| f64::from(axis) + 0.5);
            level.play_sound(
                game,
                "minecraft:block.beehive.exit",
                at,
                SoundSource::Blocks,
                0.9,
                pitch,
            );
            if let Some(key) = mod_api::__private::particle_named_key("minecraft:poof") {
                level.send_particles(
                    game,
                    &mod_api::ParticleOptions::simple(key),
                    None,
                    at,
                    [0.25; 3],
                    0.0,
                    8,
                );
            }
        }
    } else if let Some(ant) = game.mobs.iter_mut().find(|m| m.id == id) {
        let cooldown = 400 + ant.random.next_int_bound(800).unwrap_or(0);
        let raw = game
            .persist
            .raw
            .entry(id)
            .or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw {
            fields.insert("HillCooldown".into(), Tag::Int(cooldown));
        }
    }
}
