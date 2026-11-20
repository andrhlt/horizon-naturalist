use super::{Game, ItemStack};
use horizon_client::mobs::{Who, goals::naturalist_rat_goals as rat};
use horizon_core::geometry::Vec3;

pub(super) fn sense(game: &mut Game<'_>) {
    for index in 0..game.mobs.len() {
        let mob = &game.mobs[index];
        if mob.kind.id != rat::KIND || !mob.alive() || !mob.extra.tame {
            continue;
        }
        let checked = rat::state(mob).station.and_then(|pos| {
            game.world
                .loaded(pos[0], pos[2])
                .then(|| game.rat_has_container(pos, true))
        });
        rat::state_mut(&mut game.mobs[index]).station_exists = checked;
    }
}

pub(super) fn assign(game: &mut Game<'_>, id: u32, pos: [i32; 3]) {
    let Some(index) = game
        .mobs
        .iter()
        .position(|mob| mob.id == id && mob.kind.id == rat::KIND && mob.extra.tame)
    else {
        return;
    };
    if game.rat_has_container(pos, false) {
        let mob = &mut game.mobs[index];
        rat::set_station(mob, Some(pos));
        horizon_client::mobs::nav::naturalist_duck_pet::set_following(&mut mob.ai.nav, false);
        mob.extra.sitting = false;
    }
}

pub(super) fn actions(game: &mut Game<'_>) {
    for index in 0..game.mobs.len() {
        let mob = &mut game.mobs[index];
        if mob.kind.id != rat::KIND || !mob.alive() || mob.removed {
            continue;
        }
        let at = mob.position;
        match rat::state_mut(mob).action.take() {
            Some(rat::WorkAction::Harvest(pos)) => harvest(game, index, pos),
            Some(rat::WorkAction::Deposit(pos, step)) => deposit(game, index, pos, step),
            Some(rat::WorkAction::Abandon(pos)) => {
                if !game.rat_has_container(pos, false) {
                    spill(game, index, at);
                    rat::set_station(&mut game.mobs[index], None);
                }
            }
            None => {}
        }
    }
}

fn harvest(game: &mut Game<'_>, index: usize, pos: [i32; 3]) {
    if !rat::ripe(&game.mob_world_base(&[]), pos) {
        return;
    }
    let [x, y, z] = pos;
    let state = game.world.block(x, y, z);
    let name = game.res.facts().get(state).name.clone();
    let Some(replanted) = game
        .with_props(state, &[("age", "0")])
        .or_else(|| game.pack.assets.registry.default_state(&name))
    else {
        return;
    };
    let registry = game.pack.assets.registry;
    let items = world_items!(game);
    let mut drops = if let Some(sequence) = items.block_loot_sequence(registry, state) {
        let rng = game.rand_sequences.get(&sequence, game.level.seed);
        items.block_drops_exploded(registry, state, None, &[], rng, None, None, true)
    } else {
        items.block_drops_exploded(
            registry,
            state,
            None,
            &[],
            &mut game.world.random,
            None,
            None,
            true,
        )
    };
    if game.level_set(pos, replanted).is_err() {
        return;
    }
    game.level_event_at(2001, pos, state.0 as i32);
    game.post_block_game_event(
        "minecraft:block_change",
        pos,
        Some(Who::Mob(game.mobs[index].id)),
        Some(replanted),
    );
    for stack in &mut drops {
        let max = game.max_stack(stack);
        fill_cargo(&mut rat::state_mut(&mut game.mobs[index]).cargo, stack, max);
        if stack.count > 0 {
            game.pop_resource(pos, stack.clone());
        }
    }
}

fn fill_cargo(cargo: &mut Vec<Option<ItemStack>>, stack: &mut ItemStack, max: u32) {
    cargo.resize_with(rat::SLOTS, || None);
    for current in cargo.iter_mut().flatten() {
        if !current.same_item_same_components(stack) {
            continue;
        }
        let moved = stack.count.min(max.saturating_sub(current.count));
        current.count += moved;
        stack.count -= moved;
    }
    for slot in cargo.iter_mut().filter(|slot| slot.is_none()) {
        if stack.count == 0 {
            break;
        }
        let count = stack.count.min(max);
        *slot = Some(ItemStack {
            count,
            ..stack.clone()
        });
        stack.count -= count;
    }
}

fn deposit(game: &mut Game<'_>, index: usize, pos: [i32; 3], step: i32) {
    let at = game.mobs[index].position;
    let len = rat::state(&game.mobs[index]).cargo.len();
    for slot in 0..len {
        let Some(stack) = rat::state(&game.mobs[index]).cargo[slot].as_ref() else {
            continue;
        };
        let one = ItemStack {
            count: 1,
            ..stack.clone()
        };
        match game.rat_deposit(pos, one) {
            Some(true) => {
                let rat = rat::state_mut(&mut game.mobs[index]);
                let stack = rat.cargo[slot].as_mut().expect("cargo slot was occupied");
                stack.count -= 1;
                if stack.count == 0 {
                    rat.cargo[slot] = None;
                }
                rat.deposited = Some(true);
                let pitch = 1.0 + 0.05 * (step % 16) as f32;
                game.native_play_sound_in_place(
                    "minecraft:block.decorated_pot.insert",
                    at.components(),
                    6,
                    0.2,
                    pitch,
                );
                return;
            }
            Some(false) => {}
            None => {
                spill(game, index, at);
                break;
            }
        }
    }
    rat::state_mut(&mut game.mobs[index]).deposited = Some(false);
}

pub(super) fn spill(game: &mut Game<'_>, index: usize, at: Vec3) {
    let mut cargo = std::mem::take(&mut rat::state_mut(&mut game.mobs[index]).cargo);
    cargo.resize_with(rat::SLOTS, || None);
    game.drop_vehicle_contents(at.components().map(f64::floor), &cargo);
}
