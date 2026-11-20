use super::Game;
use horizon_client::{
    containers::BlockEntity,
    mobs::naturalist_ant::{ANT, Action, FoodTarget, HILL, start_cooldown, state, state_mut},
    particles::{ParticleOptions, ParticleType},
    save::{item_from_nbt, item_to_nbt},
};
use horizon_core::{geometry::Vec3, random::RandomSource};
use horizon_world::nbt::Tag;

fn uuid(tag: Option<&Tag>) -> Option<[i32; 4]> {
    match tag {
        Some(Tag::IntArray(uuid)) => uuid.as_slice().try_into().ok(),
        _ => None,
    }
}

pub(super) fn update_food_targets(game: &mut Game<'_>) {
    let items = game.res.items();
    let raw = &game.persist.raw;
    let drops = &game.dropped;
    for ant in game
        .mobs
        .iter_mut()
        .filter(|mob| mob.kind.id == ANT && mob.alive())
    {
        if state(ant).meal.is_some() || ant.ai.target.is_some() {
            state_mut(ant).food = None;
            continue;
        }
        let area = ant.bounds().inflate(Vec3::new(8.0, 4.0, 8.0));
        let following = state(ant).pursuing_food;
        state_mut(ant).food = drops
            .iter()
            .filter(|drop| {
                drop.pickup_delay == 0
                    && drop.stack.count > 0
                    && raw.get(&drop.id).and_then(|tag| tag.get("Ant")).is_none()
                    && following.map_or_else(|| area.intersects(drop.bounds()), |id| drop.id == id)
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
            .map(|drop| FoodTarget {
                id: drop.id,
                at: drop.position.components(),
            });
    }
}

fn hill_workers(game: &Game<'_>, [x, y, z]: [i32; 3], owner: Option<[i32; 4]>) -> Option<u8> {
    let row = game.pack.assets.registry.state(game.world.block(x, y, z))?;
    if row.name != HILL || row.properties.get("open").map(String::as_str) != Some("true") {
        return None;
    }
    let BlockEntity::Mod(entity) = game.world.block_entities.get(&(x, y, z))? else {
        return None;
    };
    if owner != uuid(entity.nbt.get(&"Owner".into())) {
        return None;
    }
    row.properties.get("workers")?.parse().ok()
}

fn clear_meal(game: &mut Game<'_>, id: u32) {
    if let Some(ant) = game.mobs.iter_mut().find(|mob| mob.id == id) {
        state_mut(ant).meal = None;
    }
    if let Some(Tag::Compound(fields)) = game.persist.raw.get_mut(&id) {
        fields.remove(&"CarriedFood".into());
    }
}

fn pick_up(game: &mut Game<'_>, id: u32, food: u32) {
    let Some(ant) = game
        .mobs
        .iter()
        .find(|mob| mob.id == id && mob.alive() && !mob.removed)
    else {
        return;
    };
    let Some(index) = game
        .dropped
        .iter()
        .position(|drop| drop.id == food && drop.pickup_delay == 0)
    else {
        return;
    };
    if state(ant).meal.is_some() || ant.distance_sqr(game.dropped[index].position) >= 1.0 {
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
    if let Some(ant) = game.mobs.iter_mut().find(|mob| mob.id == id) {
        state_mut(ant).meal = Some(carried_id);
        let pitch = ((ant.random.next_float() - ant.random.next_float()) * 0.7 + 1.0) * 2.0;
        game.native_play_sound_in_place("minecraft:entity.item.pickup", [x, y, z], 4, 0.2, pitch);
    }
}

pub(super) fn hill_effects(game: &mut Game<'_>, at: [i32; 3]) {
    let at = at.map(|axis| f64::from(axis) + 0.5);
    let pitch = 0.9 + game.world.random.next_float() * 0.2;
    game.native_play_sound_in_place("minecraft:block.beehive.exit", at, 4, 0.9, pitch);
    let options = ParticleOptions::simple(ParticleType::Poof);
    game.with_level(|_, level, _| {
        level.native_send_particles(&options, None, at, [0.25; 3], 0.0, 8)
    });
}

fn store_food(game: &mut Game<'_>, room: [i32; 3], id: u32, meal: u32) {
    let Some(index) = game.dropped.iter().position(|drop| drop.id == meal) else {
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
                let Some(mut stack) = item_from_nbt(entry) else {
                    continue;
                };
                if stack.count < limit && stack.same_item_same_components(&item) {
                    stack.count += 1;
                    *entry = item_to_nbt(&stack);
                    return true;
                }
            }
            if stacks.len() >= 9 {
                return false;
            }
            stacks.push(item_to_nbt(&item));
            true
        })
        .unwrap_or(false);
    game.remove_dropped(index);
    clear_meal(game, id);
    if !stored {
        game.spawn_item_own(item, room.map(|axis| f64::from(axis) + 0.5), 10);
    }
    hill_effects(game, room);
}

pub(super) fn apply_actions(game: &mut Game<'_>) {
    let pending: Vec<_> = game
        .mobs
        .iter_mut()
        .filter(|mob| mob.kind.id == ANT && mob.alive() && !mob.removed)
        .filter_map(|ant| {
            let action = state_mut(ant).action.take()?;
            Some((ant.id, ant.extra.owner, state(ant).meal, action))
        })
        .collect();
    for (id, owner, meal, action) in pending {
        match action {
            Action::PickUp(food) => pick_up(game, id, food),
            Action::Store(room) => {
                if let Some(meal) = meal
                    && hill_workers(game, room, owner).is_some()
                {
                    store_food(game, room, id, meal);
                }
            }
            Action::Enter(room) => {
                let entered = hill_workers(game, room, owner)
                    .filter(|workers| *workers < 3)
                    .is_some_and(|workers| {
                        game.native_set_block_property(room, "workers", &(workers + 1).to_string())
                    });
                if let Some(ant) = game.mobs.iter_mut().find(|mob| mob.id == id) {
                    if entered {
                        ant.removed = true;
                    } else {
                        start_cooldown(ant);
                    }
                }
                if entered {
                    hill_effects(game, room);
                }
            }
        }
    }
}

pub(super) fn is_carried_food(tag: &Tag) -> bool {
    tag.get("Ant").is_some()
        || matches!(tag.get("id"), Some(Tag::String(id)) if id.to_string_lossy() == "naturalist:carried_food")
}

pub(super) fn update_carried_items(game: &mut Game<'_>) {
    let now = game.world.game_time;
    let uuids = &game.persist.uuids;
    let raw = &mut game.persist.raw;
    let mut released = Vec::new();
    let mut restored = Vec::new();
    for drop in &mut game.dropped {
        let Some(tag) = raw.get(&drop.id).filter(|tag| is_carried_food(tag)) else {
            continue;
        };
        let owner = uuid(tag.get("Ant"));
        let ant = owner.and_then(|owner| {
            game.mobs.iter().find(|mob| {
                mob.kind.id == ANT && mob.alive() && !mob.removed && uuids.get(&mob.id) == Some(&owner)
            })
        });
        if let Some(ant) = ant {
            let saved = uuid(raw.get(&ant.id).and_then(|tag| tag.get("CarriedFood")));
            if saved.is_none() || saved.as_ref() != uuids.get(&drop.id) {
                released.push(drop.id);
                continue;
            }
            drop.previous = drop.position;
            drop.position =
                ant.position
                    .add_vector(Vec3::new(0.0, f64::from(ant.height()) + 0.05, 0.0));
            drop.velocity = Vec3::ZERO;
            drop.age = i32::from(i16::MIN);
            drop.no_gravity = true;
            restored.push((ant.id, drop.id));
            if let Some(Tag::Compound(fields)) = raw.get_mut(&drop.id) {
                fields.remove(&"NaturalistUnresolvedSince".into());
            }
        } else if let Some(Tag::Compound(fields)) = raw.get_mut(&drop.id) {
            let first = fields
                .entry("NaturalistUnresolvedSince".into())
                .or_insert(Tag::Long(now));
            if let Tag::Long(first) = first
                && now.saturating_sub(*first) >= 60
            {
                released.push(drop.id);
            }
        }
    }
    for (ant_id, drop) in restored {
        if let Some(ant) = game.mobs.iter_mut().find(|ant| ant.id == ant_id) {
            state_mut(ant).meal = Some(drop);
        }
    }
    for id in released {
        if let Some(index) = game.dropped.iter().position(|drop| drop.id == id) {
            let item = &game.dropped[index];
            let (stack, at) = (item.stack.clone(), item.position.components());
            game.remove_dropped(index);
            game.spawn_item_own(stack, at, 10);
        }
    }
}
