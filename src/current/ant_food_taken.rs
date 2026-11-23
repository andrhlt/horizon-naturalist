fn ant_food_taken(game: &mut Game<'_>, player: u128, item: mod_api::ItemEntityRef) {
    use horizon_client::mobs::Who;
    let id = item.entity().id();
    let Some(Tag::IntArray(carrier)) = game.persist.raw.get(&id).and_then(|raw| raw.get("Ant"))
    else {
        return;
    };
    let Ok(carrier) = <[i32; 4]>::try_from(carrier.as_slice()) else {
        return;
    };
    let Some(ant_id) = game.mob_by_uuid(carrier) else {
        return;
    };
    let bytes = player.to_be_bytes();
    let actor: [i32; 4] =
        std::array::from_fn(|i| i32::from_be_bytes(bytes[4 * i..4 * i + 4].try_into().unwrap()));
    let guest = game.lan.as_ref().and_then(|host| {
        host.hub
            .guests
            .iter()
            .find(|g| horizon_client::lan::playerdata::uuid_ints(g.uuid) == actor)
            .map(|g| g.conn)
    });
    if let Some(Tag::Compound(fields)) = game.persist.raw.get_mut(&ant_id) {
        fields.remove(&"CarriedFood".into());
    }
    if let Some(ant) = game
        .mobs
        .iter_mut()
        .find(|m| m.id == ant_id && m.kind.id == "naturalist:ant")
    {
        if ant.extra.owner != Some(actor) && ant.ai.target.is_none() {
            ant.ai.target = Some(Who::Player);
            ant.extra.angry_at = Some(Who::Player);
            ant.extra.anger = 500;
            ant.extra.anger_end = game.world.game_time + 500;
            if let Some(Tag::Compound(fields)) = game.persist.raw.get_mut(&ant_id) {
                fields.insert("NaturalistAngryActor".into(), Tag::IntArray(actor.to_vec()));
            }
            if let Some(conn) = guest
                && let Some(host) = &mut game.lan
            {
                host.mob_players.insert(ant_id, conn);
            }
        }
    }
}
