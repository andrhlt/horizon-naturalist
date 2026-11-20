fn elephant_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::mobs::nav::naturalist_elephants as elephant;
    use std::ops::ControlFlow;

    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let mob = &game.mobs[index];
    if !elephant::includes(&mob.kind.id) || !mob.alive() {
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
    let adult = !mob.is_baby();
    let occupied = game.riding == Some(id)
        || game.ridden_by_guest(id)
        || game
            .mobs
            .iter()
            .any(|passenger| passenger.alive() && passenger.extra.mount == Some(id));
    if adult && occupied {
        return ControlFlow::Break(Ok(false));
    }
    let actor = game.acting_player_uuid();
    let mob = &game.mobs[index];
    let owner = mob.extra.tame && mob.extra.owner == Some(actor);
    if !mob.extra.tame && !adult && item == "minecraft:cake" {
        let mob = &mut game.mobs[index];
        let state = elephant::state_mut(mob);
        state.cakes = state.cakes.saturating_add(1);
        let tamed = state.cakes >= 5;
        if tamed {
            mob.extra.owner = Some(actor);
            mob.extra.tame = true;
            mob.persistent = true;
            let entity = game.mob_entity(&game.mobs[index]);
            game.trigger_entity("minecraft:tame_animal", "entity", &entity);
            let kind = game.mobs[index].kind.id.clone();
            game.native_entity_tame(
                id,
                &kind,
                Some(horizon_client::lan::playerdata::ints_uuid(actor)),
            );
        }
        game.mob_event_particles(id, if tamed { 7 } else { 6 });
        game.consume_held();
        game.swing_arm();
        return ControlFlow::Break(Ok(true));
    }

    if item == "minecraft:melon_slice" {
        let injured_owned = owner && mob.health < mob.attrs.max_health;
        let mob = &mut game.mobs[index];
        if injured_owned {
            mob.heal(4.0);
        } else if feed_animal(mob, &mut game.world.random, Some(actor)).is_none() {
            return ControlFlow::Break(Ok(false));
        } else if mob.extra.in_love > 0 {
            let kind = mob.kind.id.clone();
            game.love_cause.insert(id, game.lan_acting);
            game.native_entity_enter_love_mode(id, &kind, game.lan_acting_uuid);
            game.mob_event_particles(id, 18);
        }
        game.consume_held();
        game.swing_arm();
        return ControlFlow::Break(Ok(true));
    }
    if !adult || !owner {
        return ControlFlow::Break(Ok(false));
    }

    let mob = &mut game.mobs[index];
    let at = mob.position.components();
    let sound = match item {
        "minecraft:shears" if mob.extra.chested || mob.extra.saddled => {
            let height = f64::from(mob.height());
            let position = mob.position;
            let removed = if mob.extra.chested {
                mob.extra.chested = false;
                let mut stacks = vec![ItemStack::new("minecraft:chest", 1)];
                stacks.extend(
                    mob.extra
                        .items
                        .iter_mut()
                        .take(elephant::CHEST_SLOTS)
                        .filter_map(Option::take),
                );
                stacks
            } else {
                let saddle = mob
                    .saddle_item()
                    .unwrap_or_else(|| ItemStack::new("minecraft:saddle", 1));
                mob.set_saddle(None);
                if let Some(slot) = mob.extra.items.get_mut(elephant::SADDLE_SLOT) {
                    *slot = None;
                }
                vec![saddle]
            };
            for stack in removed {
                game.spawn_at_location(position, height, stack);
            }
            game.damage_held(1);
            Some("minecraft:entity.sheep.shear")
        }
        "minecraft:chest" if !mob.extra.chested => {
            mob.extra.chested = true;
            mob.extra.items.resize(elephant::INVENTORY_SLOTS, None);
            game.consume_held();
            Some("minecraft:entity.mule.chest")
        }
        "minecraft:saddle" if !mob.extra.saddled => {
            let saddle = held.clone().map(|mut stack| {
                stack.count = 1;
                stack
            });
            mob.set_saddle(saddle.clone());
            mob.extra.items.resize(elephant::INVENTORY_SLOTS, None);
            mob.extra.items[elephant::SADDLE_SLOT] = saddle;
            game.consume_held();
            Some("minecraft:entity.horse.saddle")
        }
        _ => None,
    };
    if let Some(sound) = sound {
        game.audio.play(sound, Some(at), 1.0, 1.0);
        game.swing_arm();
        return ControlFlow::Break(Ok(true));
    }

    if game.keys.held(Action::Sneak) {
        game.open_mount_menu(index);
        return ControlFlow::Break(Ok(true));
    }
    if item.is_empty()
        && game.mobs[index].extra.saddled
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
