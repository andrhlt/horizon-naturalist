fn ant_food_taken(game: &mut Game<'_>, taken: &Taken, collector: Collector) {
    use horizon_client::mobs::Who;
    use horizon_client::mobs::naturalist_ant::{self, ANGER_TICKS};
    use horizon_world::nbt::Tag;
    if game.remote.is_some() {
        return;
    }
    let Taken::Item(item) = taken else { return };
    let Some(Tag::IntArray(carrier)) = game
        .persist
        .raw
        .get(&item.id)
        .and_then(|tag| tag.get("Ant"))
    else {
        return;
    };
    let Ok(carrier) = <[i32; 4]>::try_from(carrier.as_slice()) else {
        return;
    };
    let Some(id) = game.mob_by_uuid(carrier) else {
        return;
    };
    let (actor, guest) = match collector {
        Collector::Player => (game.acting_player_uuid(), game.lan_acting),
        Collector::Mob(player) => {
            let Some(guest) = game.lan.as_ref().and_then(|host| {
                host.hub.guests.iter().find(|guest| {
                    host.hub.here(guest) && guest.entity_id as u32 == player && !guest.spectator()
                })
            }) else {
                return;
            };
            (
                horizon_client::lan::playerdata::uuid_ints(guest.uuid),
                Some(guest.conn),
            )
        }
    };
    if let Some(Tag::Compound(tag)) = game.persist.raw.get_mut(&id) {
        tag.remove(&"CarriedFood".into());
    }
    if let Some(ant) = game
        .mobs
        .iter_mut()
        .find(|mob| mob.id == id && mob.kind.id == naturalist_ant::ANT)
    {
        naturalist_ant::state_mut(ant).meal = None;
        if ant.extra.owner != Some(actor) && ant.ai.target.is_none() {
            ant.ai.target = Some(Who::Player);
            ant.extra.angry_at = Some(Who::Player);
            ant.extra.anger = ANGER_TICKS;
            ant.extra.anger_end = game.world.game_time + i64::from(ANGER_TICKS);
            naturalist_ant::state_mut(ant).anger_actor = Some(actor);
            if let Some(conn) = guest
                && let Some(host) = &mut game.lan
            {
                host.mob_players.insert(id, conn);
            }
        }
    }
}
