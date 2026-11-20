use horizon_core::random::RandomSource;

use super::{BlockPos, BlockStateId, Direction, Game, Hit, ItemStack, LevelEvent, PlaceContext, Vec3};

pub(super) const ID: &str = "naturalist:glow_goop";
const LEVELS: [&str; 3] = ["1", "2", "3"];

fn layers(game: &Game<'_>, state: BlockStateId) -> Option<usize> {
    let state = game.pack.assets.registry.state(state)?;
    if state.name != ID {
        return None;
    }
    let value = state.properties.get("goop")?;
    LEVELS.iter().position(|level| *level == value).map(|index| index + 1)
}

fn next_layer(game: &Game<'_>, pos: [i32; 3]) -> Option<BlockStateId> {
    let state = game.world.block(pos[0], pos[1], pos[2]);
    let next = *LEVELS.get(layers(game, state)?)?;
    game.with_props(state, &[("goop", next)])
}

pub(super) fn layer_state(
    game: &mut Game<'_>,
    hit: &Hit,
    stack: &ItemStack,
    sneaking: bool,
) -> Option<([i32; 3], BlockStateId)> {
    if stack.item.as_ref() != ID || sneaking {
        return None;
    }
    if let Some(state) = next_layer(game, hit.pos) {
        return Some((hit.pos, state));
    }
    let face = Direction::ALL.into_iter().find(|direction| direction.normal() == hit.face)?;
    let adjacent = std::array::from_fn(|axis| hit.pos[axis] + hit.face[axis]);
    let state = next_layer(game, adjacent)?;
    // A main-hand ray sees Goop itself. An off-hand use may hit the support
    // underneath, so check whether ordinary placement would replace the clicked
    // block before merging with the adjacent Goop.
    let placement = PlaceContext {
        clicked_pos: BlockPos::new(hit.pos[0], hit.pos[1], hit.pos[2]),
        face,
        hit: Vec3::new(hit.point[0], hit.point[1], hit.point[2]),
        inside: false,
        yaw: game.yaw,
        pitch: game.pitch,
        sneaking,
        item: stack.item.to_string(),
        can_use_game_master_blocks: game.can_use_game_master_blocks(),
    };
    game.with_level(|behaviours, level, ticks| behaviours.placement_state(level, ticks, &placement))
        .is_none()
        .then_some((adjacent, state))
}

pub(super) fn stack_layer(game: &mut Game<'_>, stack: &ItemStack, clicked: [i32; 3]) -> bool {
    if stack.item.as_ref() != ID {
        return false;
    }
    let Some(hit) = game.pick() else { return false };
    let sneaking = game.keys.held(super::Mapping::Sneak);
    let Some((pos, state)) = layer_state(game, &hit, stack, sneaking) else {
        return false;
    };
    if !game.may_build() && !game.can_place_on_in_adventure(clicked) {
        return false;
    }
    if let (Some(uuid), Some(name)) = (game.lan_acting_uuid, game.lan_acting_name.clone()) {
        let mut canceled = false;
        game.native_block_place(uuid, &name, stack, pos, state, &mut canceled);
        if canceled {
            game.use_failed = true;
            game.guest_inventory_resend = true;
            return true;
        }
    }
    let at = BlockPos::new(pos[0], pos[1], pos[2]);
    let placed = game.with_level(|behaviours, level, ticks| {
        behaviours.set_block(level, ticks, at, state, 11)
    });
    if !placed {
        return false;
    }
    game.level_events.push(LevelEvent::Sound {
        pos: at,
        sound: "minecraft:block.honey_block.place".into(),
        volume: 1.0,
        pitch: 0.8,
    });
    let actor = game.sculk_actor();
    game.post_block_game_event("minecraft:block_place", pos, actor, Some(state));
    game.award_stat("used", ID, 1);
    game.trigger_block("minecraft:placed_block", pos, stack);
    game.native_place_block(stack, pos, state);
    game.consume_held();
    game.apply_use_cooldown(stack);
    let _ = game.process_level_events();
    true
}

pub(super) fn replace(game: &mut Game<'_>, context: &PlaceContext) {
    if context.item == ID {
        return;
    }
    let first = context.clicked_pos;
    let side = context.face.normal();
    let adjacent = BlockPos::new(first.x + side[0], first.y + side[1], first.z + side[2]);
    if [first, adjacent].into_iter().all(|pos| layers(game, game.world.block(pos.x, pos.y, pos.z)).is_none()) {
        return;
    }
    let replace_clicked = game.with_level(|behaviours, level, ticks| {
        behaviours.directional_can_place(level, ticks, context)
    });
    let pos = if replace_clicked { first } else { adjacent };
    let Some(count) = layers(game, game.world.block(pos.x, pos.y, pos.z)) else {
        return;
    };
    game.pop_resource([pos.x, pos.y, pos.z], ItemStack::new(ID, count as u32));
    game.with_level(|behaviours, level, ticks| {
        behaviours.remove_block(level, ticks, pos, false);
    });
}

pub(super) fn mined(game: &mut Game<'_>, pos: [i32; 3], old: BlockStateId) {
    let Some(count) = layers(game, old) else {
        return;
    };
    let at = BlockPos::new(pos[0], pos[1], pos[2]);
    let pitch = 0.9 + game.world.random.next_float() * 0.2;
    game.level_events.push(LevelEvent::Sound {
        pos: at,
        sound: "minecraft:block.honey_block.break".into(),
        volume: 0.7,
        pitch,
    });
    if count > 1 && let Some(state) = game.with_props(old, &[("goop", LEVELS[count - 2])]) {
        game.with_level(|behaviours, level, ticks| {
            behaviours.set_block(level, ticks, at, state, 2);
        });
        game.post_block_game_event("minecraft:block_destroy", pos, None, Some(old));
        game.level_event_at(2001, pos, old.0 as i32);
    }
}
