fn giraffe_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_core::random::RandomSource;
    use std::ops::ControlFlow;
    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let mob = &game.mobs[index];
    if mob.kind.id != "naturalist:giraffe" || !mob.alive() {
        return ControlFlow::Continue(());
    }
    let held = game.held().cloned();
    let item = held.as_ref().map_or("", |stack| stack.item.as_ref());
    if matches!(
        item,
        "minecraft:lead" | "minecraft:name_tag" | "minecraft:golden_dandelion"
    ) {
        return ControlFlow::Continue(());
    }

    let id = mob.id;
    let occupied = game.riding == Some(id)
        || game.ridden_by_guest(id)
        || game
            .mobs
            .iter()
            .any(|passenger| passenger.alive() && passenger.extra.mount == Some(id));
    if occupied && !mob.is_baby() {
        return ControlFlow::Break(Ok(false));
    }
    let food = game
        .items()
        .item_tags
        .get("naturalist:giraffe_food_items")
        .is_some_and(|tag| tag.contains(item));
    let actor = game.acting_player_uuid();
    let mob = &game.mobs[index];
    let owned = mob.extra.tame && mob.extra.owner == Some(actor);
    let baby = mob.is_baby();
    let freshly_tamed = !mob.extra.tame && baby && food;
    let breeding = mob.extra.age == 0 && mob.extra.in_love == 0 && food;
    let growing = baby && food && !mob.extra.age_locked;
    let healing = owned && food && mob.health < mob.attrs.max_health;
    if freshly_tamed || breeding || growing || healing {
        let mob = &mut game.mobs[index];
        let at = mob.center();
        let mut particles = None;
        let mut tamed = false;
        let horse_eat = !mob.silent && (freshly_tamed || healing || breeding && !mob.extra.tame);
        if freshly_tamed {
            let trained = mob.random.next_int_bound(3).unwrap_or(1) == 0;
            if trained {
                mob.extra.tame = true;
                mob.extra.owner = Some(actor);
                mob.persistent = true;
                tamed = true;
            }
            particles = Some(if trained { 7 } else { 6 });
        } else if healing {
            mob.heal(4.0);
        } else {
            if feed_animal(mob, &mut game.world.random, Some(actor)).is_none() {
                return ControlFlow::Break(Ok(false));
            }
            if mob.extra.in_love > 0 {
                game.love_cause.insert(id, game.lan_acting);
                game.native_entity_enter_love_mode(id, "naturalist:giraffe", game.lan_acting_uuid);
                particles = Some(18);
            }
        }
        if tamed {
            let entity = game.mob_entity(&game.mobs[index]);
            game.trigger_entity("minecraft:tame_animal", "entity", &entity);
            game.native_entity_tame(
                id,
                "naturalist:giraffe",
                Some(horizon_client::lan::playerdata::ints_uuid(actor)),
            );
        }
        if let Some(event) = particles {
            game.mob_event_particles(id, event);
        }
        if horse_eat {
            game.audio
                .play("minecraft:entity.horse.eat", Some(at), 1.0, 1.0);
        }
        if let Some(held) = &held {
            let entity = game.mob_entity(&game.mobs[index]);
            game.trigger_item_entity("minecraft:player_interacted_with_entity", held, &entity);
        }
        game.consume_held();
        game.swing_arm();
        return ControlFlow::Break(Ok(true));
    }

    if owned
        && !baby
        && item.is_empty()
        && !game.keys.held(Action::Sneak)
        && game.native_may_board(id)
    {
        let mob = &game.mobs[index];
        game.yaw = mob.yaw;
        game.pitch = mob.head_pitch;
        game.start_riding(id);
        return ControlFlow::Break(Ok(true));
    }
    ControlFlow::Break(Ok(false))
}
