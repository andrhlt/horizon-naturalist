fn snake_items(game: &mut Game<'_>) {
    use horizon_client::mobs::{Who, goals::naturalist_snake_goals as snake, neutral_anger};
    use horizon_core::geometry::{Aabb, Vec3};
    let griefing = game.ui.rules.bool("mob_griefing");
    let mut spits = Vec::new();
    for i in 0..game.mobs.len() {
        let mob = &game.mobs[i];
        if mob.kind.id != snake::KIND {
            continue;
        }
        if snake::state(&mob.ai.nav).anger_load_pending {
            let status = snake::state(&mob.ai.nav);
            let end = status.anger_end.unwrap_or(-1);
            let target = status
                .anger_uuid
                .map(|uuid| game.who_by_uuid(uuid).unwrap_or(Who::Mob(0)));
            let mob = &mut game.mobs[i];
            mob.extra.anger_end = end;
            neutral_anger::adopt_anger_end(mob, game.world.game_time);
            let status = snake::state_mut(&mut mob.ai.nav);
            status.anger_target = target;
            status.anger_load_pending = false;
        }
        let mob = &mut game.mobs[i];
        let position = mob.position;
        let yaw = f64::from(mob.yaw.to_radians());
        let pitch = f64::from(mob.head_pitch.to_radians());
        let status = snake::state_mut(&mut mob.ai.nav);
        if let Some(stack) = status.pending_spit.take() {
            let x = position.x - yaw.sin() * pitch.cos();
            let z = position.z + yaw.cos() * pitch.cos();
            spits.push((mob.id, stack, [x, position.y + 1.0, z], !mob.silent));
        }
        status.food_position = None;
        if !mob.alive() || mob.extra.main_hand.is_some() {
            continue;
        }
        let search = mob.bounds().inflate(Vec3::new(8.0, 8.0, 8.0));
        let pickup = mob.bounds().inflate(Vec3::new(1.0, 0.0, 1.0));
        let is_food = |item: &horizon_client::entities::ItemEntity| {
            snake::FOOD.contains(&item.stack.item.as_ref())
        };
        let bounds = |item: &horizon_client::entities::ItemEntity| {
            Aabb::new(
                item.position,
                item.position.add_vector(Vec3::new(0.25, 0.25, 0.25)),
            )
        };
        snake::state_mut(&mut mob.ai.nav).food_position = game
            .dropped
            .iter()
            .find(|item| search.intersects(bounds(item)) && is_food(item))
            .map(|item| item.position);
        if griefing
            && mob.extra.pick_up_loot
            && !mob.no_ai
            && let Some(item) = game.dropped.iter().position(|item| {
                item.pickup_delay <= 0 && pickup.intersects(bounds(item)) && is_food(item)
            })
        {
            let stack = game.remove_dropped(item).stack;
            let mob = &mut game.mobs[i];
            mob.extra.set_main_hand_stack(Some(stack));
            horizon_client::mobs::finalize::set_drop_chance(
                mob,
                horizon_client::mobs::finalize::MAINHAND,
                2.0,
            );
            mob.persistent = true;
        }
    }
    for (id, stack, at, audible) in spits {
        let thrower = game.uuid_for(id);
        game.spawn_item_own(stack, at, 80);
        if let Some(item) = game.just_spawned_item() {
            item.thrower = Some(thrower);
        }
        if audible {
            game.audio
                .play("minecraft:entity.fox.spit", Some(at), 1.0, 1.0);
        }
    }
}
