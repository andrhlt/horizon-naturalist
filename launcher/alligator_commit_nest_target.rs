fn alligator_commit_nest_target(game: &mut Game<'_>) {
    use horizon_client::mobs::{Who, goals::naturalist_alligator_goals as alligator};

    let Some(mut lan) = game.lan.take() else { return };
    let mut bites = Vec::new();
    for mob in &mut game.mobs {
        if mob.kind.id == alligator::KIND
            && let Some((actor, events)) = alligator::take_nest_actor(mob)
            && mob.alive() && mob.ai.target == Some(Who::Player)
        {
            match actor {
                Some(connection) => { lan.mob_players.insert(mob.id, connection); }
                None => { lan.mob_players.remove(&mob.id); }
            }
            bites.extend(events.into_iter().map(|event| (actor, mob.id, event)));
        }
    }
    for (actor, mob, event) in bites {
        if let Some(connection) = actor {
            game.guest_hit_by(&mut lan, connection, |game| game.guest_mob_event(mob, event));
        } else {
            game.guest_mob_event(mob, event);
        }
    }
    game.lan = Some(lan);
}
