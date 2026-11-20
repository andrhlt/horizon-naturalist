use super::Game;
use horizon_client::actions::Action;
use horizon_client::{
    mobs::{naturalist_ostrich as ostrich, nav::naturalist_duck_pet as pet, render::DYE_NAMES},
    text::Text,
};
use horizon_core::random::RandomSource;

pub(super) fn interact(game: &mut Game<'_>, index: usize) -> bool {
    let item = game.held().cloned();
    let id = item.as_ref().map_or("", |stack| stack.item.as_ref());
    let actor = game.acting_player_uuid();
    let mob = &game.mobs[index];
    let tamed = mob.extra.tame;
    let owned = tamed && mob.extra.owner == Some(actor);
    let baby = mob.is_baby();
    let at = mob.center();
    let entity = mob.id;
    let sneaking = game.keys.held(Action::Sneak)
        || game.keys.raw(&winit::keyboard::KeyCode::ShiftRight);

    if id == "naturalist:whistle" && owned && !sneaking && !game.cooldowns.is_on_cooldown(id) {
        let mob = &mut game.mobs[index];
        let follow = !pet::is_following(&mob.ai.nav);
        pet::set_following(&mut mob.ai.nav, follow);
        mob.extra.sitting = false;
        if !follow {
            mob.ai.target = None;
        }
        let name = mob.custom_name.as_deref().unwrap_or("Ostrich").to_owned();
        let mode = Text::translate(
            if follow { "naturalist.whistle.following" } else { "naturalist.whistle.wandering" }, &[],
        );
        game.send_overlay_message(Text::translate("naturalist.whistle.message", &[Text::literal(name), mode]));
        game.add_cooldown(id.into(), 20);
        game.native_play_sound_in_place("naturalist:item.whistle", at, 4, 0.8, 1.0);
        return true;
    }

    if owned && mob.extra.saddled && id == "minecraft:shears" {
        let position = mob.position;
        game.mobs[index].set_saddle(None);
        game.spawn_at_location(position, 0.0, horizon_client::items::ItemStack::new("minecraft:saddle", 1));
        game.damage_held(1);
        game.native_play_sound_in_place("minecraft:entity.sheep.shear", at, 4, 1.0, 1.0);
        let actor_context = game.who_context(Some(horizon_client::mobs::Who::Player), None);
        game.post_game_event("minecraft:shear", position.components(), actor_context);
        return true;
    }

    if owned && id == "minecraft:shears" && mob.extra.color <= 15 {
        game.mobs[index].extra.color = u8::MAX;
        game.damage_held(1);
        game.native_play_sound_in_place("minecraft:entity.sheep.shear", at, 4, 1.0, 1.0);
        return true;
    }
    if owned && let Some(color) = id.strip_prefix("minecraft:").and_then(|value| value.strip_suffix("_dye"))
        .and_then(|color| DYE_NAMES.iter().position(|name| *name == color))
        && usize::from(mob.extra.color) != color
    {
        game.mobs[index].extra.color = color as u8;
        game.consume_held();
        game.native_play_sound_in_place("minecraft:item.dye.use", at, 4, 1.0, 1.0);
        return true;
    }
    if id == "minecraft:saddle" && tamed && !baby && !mob.extra.saddled {
        let position = mob.position.components();
        game.mobs[index].set_saddle(item);
        game.consume_held();
        game.native_play_sound_in_place("minecraft:entity.horse.saddle", at, 4, 0.5, 1.0);
        game.mob_equip_event(entity, position, true);
        return true;
    }

    let food = ostrich::FOOD.contains(&id);
    if food && baby {
        let mob = &mut game.mobs[index];
        if tamed {
            let age = ((-mob.extra.age / 20) as f32 * 0.1) as i32;
            horizon_client::mobs::ai::age_up(mob, age, true);
        } else {
            mob.extra.tame = true;
            mob.extra.owner = Some(actor);
            mob.extra.sitting = true;
            mob.ai.target = None;
            mob.nav_stop();
            let target = game.mob_entity(&game.mobs[index]);
            game.trigger_entity("minecraft:tame_animal", "entity", &target);
            game.mob_event_particles(entity, 7);
        }
        game.consume_held();
        return true;
    }
    if food && tamed && mob.health < mob.attrs.max_health {
        game.mobs[index].heal(2.0);
        game.consume_held();
        return true;
    }
    if owned && !baby && !food && mob.extra.saddled && !sneaking && !mob.extra.sitting_pose
        && game.riding != Some(entity) && !game.ridden_by_guest(entity)
        && !game.mobs.iter().any(|other| other.extra.mount == Some(entity) && other.alive())
    {
        if game.native_may_board(entity) {
            game.yaw = game.mobs[index].yaw;
            game.pitch = game.mobs[index].head_pitch;
            game.mount.jump_ticks = 0;
            game.mount.jump_scale = 0.0;
            game.start_riding(entity);
        }
        return true;
    }
    if food && !ostrich::state(mob).has_egg
        && horizon_client::mobs::ai::feed_animal(&mut game.mobs[index], &mut game.world.random, Some(actor)).is_some()
    {
        game.love_cause.insert(entity, game.lan_acting);
        game.native_entity_enter_love_mode(entity, ostrich::KIND, game.lan_acting_uuid);
        game.mob_event_particles(entity, 18);
        game.consume_held();
        return true;
    }
    if owned {
        let mob = &mut game.mobs[index];
        mob.extra.sitting = !mob.extra.sitting;
        mob.ai.jump = false;
        mob.ai.target = None;
        mob.nav_stop();
        return true;
    }
    false
}

pub(super) fn egg_destroyed(game: &mut Game<'_>, position: [i32; 3]) {
    if game.creative {
        return;
    }
    let actor = game.acting_player_uuid();
    disturb_owned_egg(game, position, Some(actor));
}

pub(super) fn egg_trampled(game: &mut Game<'_>, position: [i32; 3]) {
    let actor = (!game.creative && !game.spectator()).then(|| game.acting_player_uuid());
    disturb_owned_egg(game, position, actor);
}

fn disturb_owned_egg(game: &mut Game<'_>, position: [i32; 3], actor: Option<[i32; 4]>) {
    let now = game.world.game_time;
    for mob in &mut game.mobs {
        if mob.kind.id != ostrich::KIND || !mob.alive() || mob.is_baby() || mob.extra.tame {
            continue;
        }
        let center = mob.position.components();
        if (center[0] - f64::from(position[0]) - 0.5).abs() > 16.5
            || (center[1] - f64::from(position[1]) - 0.5).abs() > 8.5
            || (center[2] - f64::from(position[2]) - 0.5).abs() > 16.5
        {
            continue;
        }
        let state = ostrich::state_mut(mob);
        let Some(index) = state.owned_eggs.iter().position(|egg| *egg == position) else { continue };
        state.owned_eggs.remove(index);
        let Some(actor) = actor else { continue };
        let duration = 400 + mob.random.next_int_bound(381).unwrap_or(0);
        let state = ostrich::state_mut(mob);
        state.anger_actor = Some(actor);
        state.anger_end = Some(now + i64::from(duration));
        mob.ai.target = Some(horizon_client::mobs::Who::Player);
    }
}

pub(super) fn set_seat(game: &mut Game<'_>) {
    use horizon_client::mobs::riders;
    let Some(mob) = game.riding.and_then(|id| game.mobs.iter().find(|mob| mob.id == id))
        .filter(|mob| mob.kind.id == ostrich::KIND) else { return };
    let at = riders::seat(riders::passenger_attachment(mob), f64::from(0.6_f32), mob.position, mob.yaw);
    if let Some(player) = game.player.as_mut() {
        game.previous_position = if game.previous_position == player.position { at } else { game.previous_position };
        player.position = at;
    }
}

pub(super) fn update_visible_players(game: &mut Game<'_>) {
    let guests = game.lan_guest_views();
    if guests.is_empty() {
        for mob in game.mobs.iter_mut().filter(|mob| mob.kind.id == ostrich::KIND) {
            ostrich::state_mut(mob).all_players_visibility = None;
        }
        return;
    }
    let world = game.mob_world_base(&[]);
    let players = world.player.iter().chain(guests.iter().map(|(_, player)| player));
    let sightings: Vec<_> = game.mobs.iter().enumerate()
        .filter(|(_, mob)| mob.kind.id == ostrich::KIND && !mob.is_baby() && !mob.extra.tame)
        .map(|(index, mob)| {
            (index, players.clone().any(|player| ostrich::visible_player(mob, &world, player)))
        }).collect();
    for (index, visible) in sightings {
        ostrich::state_mut(&mut game.mobs[index]).all_players_visibility = Some(visible);
    }
}

pub(super) fn place_egg_against_egg(
    game: &mut Game<'_>,
    stack: &horizon_client::items::ItemStack,
    position: [i32; 3],
    face: [i32; 3],
) -> bool {
    use super::{BlockPos, Direction, PlaceContext, PlaceOutcome, Vec3};
    if stack.item.as_ref() != ostrich::EGG
        || game.keys.held(Action::Sneak)
        || !game.pack.assets.registry.state(game.world.block(position[0], position[1], position[2]))
            .is_some_and(|row| row.name == ostrich::EGG)
        || !(game.may_build() || game.can_place_on_in_adventure(position))
    {
        return false;
    }
    let Some(hit) = game.pick() else { return false };
    let face = match face {
        [0, 1, 0] => Direction::Up,
        [0, -1, 0] => Direction::Down,
        [0, 0, -1] => Direction::North,
        [0, 0, 1] => Direction::South,
        [-1, 0, 0] => Direction::West,
        [1, 0, 0] => Direction::East,
        _ => return false,
    };
    let context = PlaceContext {
        clicked_pos: BlockPos::new(position[0], position[1], position[2]),
        face,
        hit: Vec3::new(hit.point[0], hit.point[1], hit.point[2]),
        inside: false,
        yaw: game.yaw,
        pitch: game.pitch,
        // The Turtle Egg template's special merging rule disables itself with
        // this flag. The Java Ostrich Egg always takes the adjacent position.
        sneaking: true,
        item: stack.item.to_string(),
        can_use_game_master_blocks: game.can_use_game_master_blocks(),
    };
    if game.block_place_canceled(stack, &context) {
        game.use_failed = true;
        game.guest_inventory_resend = true;
        return true;
    }
    let result = game.with_level(|behaviours, level, ticks| behaviours.place_block(level, ticks, &context));
    if let PlaceOutcome::Placed { pos, state } = result {
        let target = [pos.x, pos.y, pos.z];
        let state = game.apply_item_block_state(target, state, stack).unwrap_or(state);
        let _ = game.process_level_events();
        game.award_stat("used", &stack.item, 1);
        game.trigger_block("minecraft:placed_block", target, stack);
        game.native_place_block(stack, target, state);
        game.consume_held();
        game.apply_use_cooldown(stack);
    } else {
        let _ = game.process_level_events();
        game.use_failed = true;
        game.guest_inventory_resend = true;
    }
    true
}
