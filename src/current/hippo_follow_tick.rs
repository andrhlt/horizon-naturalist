fn hippo_follow_tick(game: &mut Game<'_>, entity: u32, kind: &str) {
    use crate::game::naturalist_duck_follow::{DuckFollow, DuckSit, OWNER_VIEW, OwnerView};
    use mod_api::{Holder, Host, prelude::ServicesServerHost};
    if kind != "naturalist:hippo" {
        return;
    }
    let Some(entity_ref) = game.entity(entity) else {
        return;
    };
    let Some(handle) = game.mob(entity_ref) else {
        return;
    };

    let raw = game.persist.raw.get(&entity);
    let saved_owner = raw.and_then(horizon_client::entity_nbt::owner_uuid);
    let saved_sitting = raw
        .and_then(|tag| tag.get("Sitting"))
        .and_then(horizon_world::datafix::dynamic::as_number)
        .is_some_and(|value| value.int_value() != 0);
    let following = raw
        .and_then(|tag| tag.get("FollowingOwner"))
        .and_then(horizon_world::datafix::dynamic::as_number)
        .map_or(true, |value| value.int_value() != 0);
    let owner_uuid = game
        .mobs
        .iter()
        .find(|mob| mob.id == entity)
        .and_then(|mob| mob.extra.owner)
        .or(saved_owner);
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
        if let Some(owner) = owner_uuid {
            mob.extra.owner = Some(owner);
            mob.extra.tame = true;
            mob.extra.sitting = saved_sitting;
            horizon_client::mobs::tame::restore_sitting(mob, saved_sitting);
        }
    }

    let has = |name| {
        game.mobs
            .iter()
            .find(|mob| mob.id == entity)
            .is_some_and(|mob| mob.ai.goals.has(name))
    };
    if has("FollowOwner") {
        handle.remove_goal(game, "FollowOwner");
    }
    let holder = Holder::Entity(entity);
    let level = entity_ref.level(game);
    let view = owner_uuid.and_then(|uuid| {
        game.players()
            .server_players()
            .into_iter()
            .find(|player| horizon_client::lan::playerdata::uuid_ints(player.uuid()) == uuid)
            .and_then(|player| {
                let owner = player.entity();
                if owner.level(game) != level
                    || game
                        .living(owner)
                        .and_then(|living| living.health(game))
                        .is_none_or(|health| health <= 0.0)
                {
                    return None;
                }
                let [x, y, z] = owner.position(game)?;
                let eye = owner.eye_position(game)?[1];
                Some(OwnerView {
                    position: horizon_core::geometry::Vec3::new(x, y, z),
                    eye,
                    spectator: player.is_spectator(game)?,
                    attacked: game
                        .living(owner)
                        .and_then(|living| living.last_attacker(game))
                        .is_some(),
                    uuid,
                })
            })
    });
    let cell = match OWNER_VIEW.get_existing(&mut *game, holder).ok().flatten() {
        Some(cell) => cell,
        None => {
            let Ok(cell) = OWNER_VIEW.get(&mut *game, holder) else {
                return;
            };
            if OWNER_VIEW.set(&mut *game, holder, cell.clone()).is_err() {
                return;
            }
            cell
        }
    };
    if let Ok(mut state) = cell.write() {
        *state = view;
    }

    if !has("NaturalistDuckSit") {
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
            mob.ai.goals.add(
                1,
                DuckSit {
                    owner_view: cell.clone(),
                },
            );
        }
    }
    if !following || owner_uuid.is_none() {
        if has("NaturalistDuckFollowOwner") {
            handle.remove_goal(game, "NaturalistDuckFollowOwner");
        }
        return;
    }
    if !has("NaturalistDuckFollowOwner") {
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
            mob.ai.goals.add(4, DuckFollow::new(cell));
        }
    }
}
