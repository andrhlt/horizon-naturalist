fn snake_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::{
        Who, ai,
        goals::{Cx, naturalist_snake_goals as snake},
        spawn_selection::max_cluster_size,
    };
    use horizon_core::random::{LegacyRandom, RandomSource};
    use std::{collections::BTreeMap, ops::ControlFlow};

    let Some(query) = command.strip_prefix("naturalist_fixture_snake ") else {
        return ControlFlow::Continue(());
    };
    let Some(index) = game.mobs.iter().position(|mob| {
        mob.kind.id == snake::KIND && mob.alive() && mob.is_baby() == (query == "baby")
    }) else {
        return ControlFlow::Break(false);
    };
    let world = game.mob_world_base(&[]);
    let mob = &game.mobs[index];
    let result = if query == "gear" {
        let mut sample = mob.clone();
        let mut counts = BTreeMap::new();
        for seed in 0..500_i64 {
            sample.random.set_seed(seed * 3337);
            sample.extra.set_main_hand_stack(None);
            snake::spawn_equipment(&mut sample);
            let key = sample
                .extra
                .main_hand
                .as_deref()
                .unwrap_or("empty")
                .to_owned();
            *counts.entry(key).or_insert(0) += 1;
        }
        serde_json::json!(counts)
    } else if query == "eat" {
        let mut sample = mob.clone();
        sample
            .extra
            .set_main_hand_stack(Some(ItemStack::new("minecraft:chicken", 3)));
        let mut events = Vec::new();
        let mut count = Vec::new();
        let mut has_item = Vec::new();
        for tick in 1..=6001 {
            snake::step(&mut sample, &world, &mut events);
            if matches!(tick, 1 | 5999 | 6000 | 6001) {
                count.push(sample.extra.counter);
                has_item.push(sample.extra.main_hand.is_some());
            }
        }
        let eat_event = events.iter().any(|event| {
            matches!(
                event,
                horizon_client::mobs::MobEvent::GameEvent {
                    event: "minecraft:eat",
                    ..
                }
            )
        });
        serde_json::json!({"eat_counter": count, "eat_main_hand": has_item, "eat_event": eat_event})
    } else if let Some(seed) = query
        .strip_prefix("seed ")
        .and_then(|value| value.parse::<i64>().ok())
    {
        game.mobs[index].random.set_seed(seed);
        serde_json::json!(true)
    } else if query == "bite" {
        let Some(victim) = game
            .mobs
            .iter()
            .find(|mob| mob.kind.id == "minecraft:cow" && mob.alive())
        else {
            return ControlFlow::Break(false);
        };
        let victim_id = victim.id;
        let mut sample = mob.clone();
        let mut events = Vec::new();
        let mut random = LegacyRandom::new(0);
        ai::do_hurt_target(
            &mut sample,
            Who::Mob(victim_id),
            &mut Cx {
                w: &world,
                lr: &mut random,
                events: &mut events,
            },
        );
        let Ok(()) = game.handle_mob_events(events) else {
            return ControlFlow::Break(false);
        };
        let Some(victim) = game.mobs.iter().find(|mob| mob.id == victim_id) else {
            return ControlFlow::Break(false);
        };
        serde_json::json!({
            "poison": victim.extra.effects.iter().find(|entry| entry.0 == "minecraft:poison").map_or(0, |entry| entry.1),
            "health": victim.health,
        })
    } else {
        let (width, height, eye) = mob.dims();
        let dims = [width, height, eye].map(|value| (value * 10000.0).round() as i32);
        let pose = snake::state(&mob.ai.nav);
        let spit = game
            .dropped
            .iter()
            .find(|item| item.thrower.is_some())
            .map(|item| {
                serde_json::json!([
                    item.stack.item.as_ref(),
                    item.stack.count,
                    item.pickup_delay
                ])
            });
        serde_json::json!({
            "dims": dims, "food_ids": mob.kind.food.as_ref(), "tame_ids": snake::TAME_FOOD,
            "cluster": max_cluster_size(mob.species()), "voice_volume": ai::sound_volume(mob),
            "clock": world.day_time, "can_sleep": snake::can_sleep(mob, &world),
            "sleeping": pose.sleeping, "rattling": pose.rattling, "rattlesnake": mob.extra.variant == 2,
            "climbing": mob.climbing, "jump_power": mob.jump_power(&world),
            "tamed": mob.extra.tame, "sitting": mob.extra.sitting, "in_love": mob.extra.in_love > 0,
            "main_hand_count": mob.extra.main_hand_stack().map_or(0, |stack| stack.count),
            "found_food": pose.food_position.is_some(), "pos": mob.position.components(),
            "spit": spit,
        })
    };

    game.feedback(Text::literal(format!("Snake fixture {result}")));
    ControlFlow::Break(true)
}
