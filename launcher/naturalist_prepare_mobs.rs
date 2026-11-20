fn naturalist_prepare_mobs(game: &mut Game<'_>) {
    use horizon_client::mobs::nav::naturalist_bird_navigation::{Seed, foraging, section_order};
    use horizon_core::geometry::Vec3;

    use horizon_client::mobs::{equine::Rider, nav::naturalist_elephants as elephant};
    use super::super::persist::PassengerSlot;

    prepare_capybara_owners(game);
    carry_pachyderm_alerts(game);
    restore_pachyderm_anger(game);
    for index in 0..game.mobs.len() {
        let mob = &game.mobs[index];
        if !elephant::includes(&mob.kind.id) {
            continue;
        }
        let passenger = if mob.rider.is_some() {
            None
        } else if let Some(slots) = game.persist.passenger_slots.get(&mob.id) {
            match slots.first() {
                Some(PassengerSlot::Live(id)) => Some(*id),
                _ => None,
            }
        } else {
            mob.extra.rider
        };
        let rider = passenger
            .and_then(|id| game.mobs.iter().find(|mob| mob.id == id && !mob.removed))
            .map(|mob| Rider {
                strafe: mob.ai.xxa,
                forward: mob.ai.zza,
                yaw: mob.yaw,
                pitch: mob.head_pitch,
                ..Rider::default()
            });
        elephant::state_mut(&mut game.mobs[index]).mob_rider = rider;
    }

    if !game.mobs.iter().any(|mob| mob.kind.id == "naturalist:bird" && !mob.extra.tame) {
        return;
    }
    let food = game.items().item_tags.get("naturalist:bird_food_items").cloned();
    let edible = |item: &str| {
        food.as_ref().map_or_else(
            || matches!(item, "minecraft:wheat_seeds" | "minecraft:pumpkin_seeds" | "minecraft:melon_seeds" | "minecraft:beetroot_seeds"),
            |tag| tag.contains(item),
        )
    };
    for mob in &mut game.mobs {
        if mob.kind.id != "naturalist:bird" || mob.extra.tame || !mob.alive() || mob.removed {
            continue;
        }
        let bounds = mob.bounds().inflate(Vec3::new(8.0, 4.0, 8.0));
        let target = foraging(&mut mob.ai.nav).as_ref().and_then(|state| state.target);
        let mut first = None;
        let mut selected = None;
        for drop in game.dropped.iter().filter(|drop| drop.stack.count > 0 && edible(&drop.stack.item)) {
            let seed = Seed { id: drop.id, position: drop.position };
            if bounds.intersects(drop.bounds()) {
                let section = [seed.position.x, seed.position.y, seed.position.z]
                    .map(|p| (p.floor() as i32) >> 4);
                let key = section_order(section, drop.join.get());
                if first.is_none_or(|(old, _)| key < old) {
                    first = Some((key, seed));
                }
            }
            if Some(seed.id) == target {
                selected = Some(seed);
            }
        }
        let state = foraging(&mut mob.ai.nav).get_or_insert_with(Default::default);
        state.first = first.map(|(_, seed)| seed);
        state.selected = selected;
    }
}

fn restore_pachyderm_anger(game: &mut Game<'_>) {
    use horizon_client::{lan::playerdata::uuid_ints, mobs::{Who, nav::naturalist_elephants as elephant}};

    for index in 0..game.mobs.len() {
        let mob = &game.mobs[index];
        if !elephant::includes(&mob.kind.id) || !mob.alive() {
            continue;
        }
        let status = elephant::state(mob);
        let anger_target = status.anger_target;
        let anger_uuid = status.anger_uuid;
        let pending_load = status.restore_anger;
        let uuid_is_present_player = anger_uuid.is_some_and(|uuid| {
            game.player.is_some() && game.level.player_uuid == Some(uuid)
                || game.lan.as_ref().is_some_and(|host| host.hub.guests.iter().any(|guest| {
                    guest.loaded && host.hub.here(guest) && uuid_ints(guest.uuid) == uuid
                }))
        });
        let loaded_target = pending_load.then(|| {
            if uuid_is_present_player {
                Some(Who::Player)
            } else {
                anger_uuid.and_then(|uuid| game.who_by_uuid(uuid))
                    .filter(|target| *target != Who::Mob(mob.id))
            }
        });
        let hurt_player_uuid = (mob.ai.last_hurt_by == Some(Who::Player)
            && i64::from(mob.age) - i64::from(mob.ai.last_hurt_by_time) <= 100)
            .then(|| game.last_hurt_by_player.get(&mob.id).copied())
            .flatten()
            .and_then(|conn| game.player_uuid_of(conn));
        let mob_target_uuid = match loaded_target.flatten().or(anger_target) {
            Some(Who::Mob(id)) if game.mobs.iter().any(|actor| actor.id == id) => {
                // Record the actor's actual UUID while this dimension owns it.
                Some(game.uuid_for(id))
            }
            _ => None,
        };
        let mob = &mut game.mobs[index];
        let status = elephant::state_mut(mob);
        status.hurt_player_uuid = hurt_player_uuid;
        if let Some(uuid) = mob_target_uuid {
            status.anger_uuid = Some(uuid);
        }
        if let Some(target) = loaded_target {
            status.restore_anger = false;
            status.anger_target = target;
            mob.ai.target = target;
        } else if uuid_is_present_player && anger_target.is_none() {
            status.anger_target = Some(Who::Player);
        } else if anger_target == Some(Who::Player) && !uuid_is_present_player {
            // An absent player stays the saved reference until the timer ends.
            // The engine's shared Who::Player value must not name a bystander.
            status.anger_target = None;
            status.hurt_player_uuid = None;
            mob.ai.target = None;
            mob.ai.forget_hurt_by(mob.age as i32);
        }
        elephant::prepare_anger(mob, game.world.game_time);
    }
}

fn carry_pachyderm_alerts(game: &mut Game<'_>) {
    use horizon_client::mobs::{Who, nav::naturalist_elephants as elephant};

    let mut alerts = Vec::new();
    for mob in &mut game.mobs {
        if elephant::includes(&mob.kind.id) {
            alerts.append(&mut elephant::state_mut(mob).player_alerts);
        }
    }
    for (id, player) in alerts {
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id)
            && elephant::includes(&mob.kind.id)
            && mob.ai.target == Some(Who::Player)
        {
            elephant::state_mut(mob).delegated_player_uuid = Some(player);
        }
    }
}

fn prepare_capybara_owners(game: &mut Game<'_>) {
    use horizon_client::{lan::playerdata::uuid_ints, mobs::tame::naturalist_duck_pet_goals::set_capybara_owner_view};
    use std::{collections::HashSet, sync::Arc};

    let capybara = |mob: &horizon_client::mobs::Mob| mob.kind.id == "naturalist:capybara";
    let mut views = Vec::new();
    if let Some(host) = game.lan.as_ref() {
        let wanted: HashSet<_> = game.mobs.iter()
            .filter(|mob| capybara(mob) && mob.extra.tame)
            .filter_map(|mob| mob.extra.owner)
            .collect();
        if !wanted.is_empty() {
            let guests: Vec<_> = host.hub.guests.iter()
                .filter(|guest| wanted.contains(&uuid_ints(guest.uuid)))
                .map(|guest| guest.conn)
                .collect();
            views.extend(game.player_view().filter(|owner| wanted.contains(&owner.uuid)).map(Arc::new));
            views.extend(guests.into_iter().filter_map(|guest| game.lan_guest_view(guest)).map(Arc::new));
        }
    }
    for mob in game.mobs.iter_mut().filter(|mob| capybara(mob)) {
        let owner = views.iter()
            .find(|owner| mob.extra.tame && mob.extra.owner == Some(owner.uuid))
            .cloned();
        set_capybara_owner_view(mob, owner);
    }
}
