fn bird_consume_seeds(game: &mut Game<'_>) {
    use horizon_client::{lan::playerdata::ints_uuid, mobs::nav::naturalist_bird_navigation::foraging};
    use horizon_core::random::RandomSource;

    for index in 0..game.mobs.len() {
        let mob = &mut game.mobs[index];
        if mob.kind.id != "naturalist:bird" || !game.creatures.ticked.contains(&mob.id) {
            continue;
        }
        let request = foraging(&mut mob.ai.nav).as_mut().and_then(|state| state.consume.take());
        let Some(drop) = request.and_then(|id| game.dropped.iter().position(|item| item.id == id && item.stack.count > 0)) else {
            continue;
        };
        if !mob.alive() || mob.removed || mob.extra.tame {
            continue;
        }
        let seed = &game.dropped[drop];
        let owner = seed.thrower.filter(|&uuid| game.level_players_iter().any(|player| player.uuid == ints_uuid(uuid)));
        let stack = seed.stack.clone();
        let position = seed.position;
        let mob = &mut game.mobs[index];
        let pitch = (!mob.silent).then(|| 1.0 + (mob.random.next_float() - mob.random.next_float()) * 0.2);
        let tamed = owner.filter(|_| mob.random.next_int_bound(10).unwrap_or(1) == 0);
        let id = mob.id;
        let at = [mob.position.x, mob.position.y, mob.position.z];
        if let Some(owner) = tamed {
            mob.extra.tame = true;
            mob.extra.owner = Some(owner);
            mob.persistent = true;
        }
        if let Some(pitch) = pitch {
            game.audio.play("naturalist:entity.bird.eat", Some(at), 1.0, pitch);
        }
        let particle = game.item_particle(&stack);
        let at = [position.x, position.y + 0.15, position.z];
        let spread = [0.15, 0.1, 0.15];
        game.particles.send_particles(&particle, at, spread, 0.02, 8);
        game.lan_level_particles(None, &particle, at, spread, 0.02, 8, None);
        game.dropped[drop].stack.count -= 1;
        if game.dropped[drop].stack.count == 0 {
            game.remove_dropped(drop);
        }
        game.mob_event_particles(id, if tamed.is_some() { 7 } else { 6 });
        if let Some(owner) = tamed {
            use super::commands::Target;
            let who = if Some(owner) == game.level.player_uuid {
                Some(Target::Player)
            } else {
                game.lan.as_ref().and_then(|server| {
                    server.hub.guests.iter()
                        .find(|guest| server.hub.here(guest) && guest.uuid == ints_uuid(owner))
                        .map(|guest| Target::Guest(guest.conn))
                })
            };
            if let Some(who) = who {
                let view = game.mob_entity(&game.mobs[index]);
                game.for_player(who, |g| g.trigger_entity("minecraft:tame_animal", "entity", &view));
            }
        }
    }
}
