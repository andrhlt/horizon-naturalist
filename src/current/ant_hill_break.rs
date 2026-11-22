fn ant_hill_player_break(
    game: &mut Game<'_>,
    pos: [i32; 3],
    state: BlockStateId,
    canceled: &mut bool,
    exp: &mut Option<u32>,
    original: impl FnOnce(&mut Game<'_>, [i32; 3], BlockStateId, &mut bool, &mut Option<u32>) -> (),
) {
    use horizon_client::mobs::Who;
    use horizon_core::{nbt::Tag, random::RandomSource};
    original(game, pos, state, canceled, exp);
    if *canceled {
        return;
    }
    let Some(row) = game.pack.assets.registry.state(state) else {
        return;
    };
    if row.name != "naturalist:ant_hill" {
        return;
    }
    let workers = row
        .properties
        .get("workers")
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(0);
    let has_queen = row.properties.get("has_queen").map(String::as_str) == Some("true");
    if let Some(level) = game.level(game.dim.id()) {
        let at = pos.map(|axis| f64::from(axis) + 0.5);
        level.play_sound(
            game,
            "minecraft:block.azalea.break",
            at,
            mod_api::SoundSource::Blocks,
            0.9,
            1.0,
        );
        if (workers > 0 || has_queen)
            && let Some(key) = mod_api::__private::particle_named_key("minecraft:poof")
        {
            level.send_particles(
                game,
                &mod_api::ParticleOptions::simple(key),
                None,
                at,
                [0.25; 3],
                0.0,
                8,
            );
        }
    }
    let owner = match game.world.block_entities.get(&(pos[0], pos[1], pos[2])) {
        Some(horizon_client::chunk::block_entity::BlockEntity::Mod(hill)) => {
            match hill.nbt.get(&"Owner".into()) {
                Some(Tag::IntArray(id)) => <[i32; 4]>::try_from(id.as_slice()).ok(),
                _ => None,
            }
        }
        _ => None,
    };
    let actor = game.acting_player_uuid();
    let guest = game.lan.as_ref().and_then(|host| {
        host.hub
            .guests
            .iter()
            .find(|g| horizon_client::lan::playerdata::uuid_ints(g.uuid) == actor)
            .map(|g| g.conn)
    });
    for _ in 0..workers {
        let at = [
            f64::from(pos[0]) + 0.25 + game.world.random.next_double() * 0.5,
            f64::from(pos[1]),
            f64::from(pos[2]) + 0.25 + game.world.random.next_double() * 0.5,
        ];
        let Some(id) = game.spawn_mob_raw("naturalist:ant", at, false) else {
            continue;
        };
        let Some(ant) = game.mobs.iter_mut().find(|m| m.id == id) else {
            continue;
        };
        ant.extra.owner = owner;
        ant.extra.tame = owner.is_some();
        let cooldown = 400 + ant.random.next_int_bound(800).unwrap_or(0);
        if owner != Some(actor) {
            ant.ai.target = Some(Who::Player);
            ant.extra.angry_at = Some(Who::Player);
            ant.extra.anger = 500;
            ant.extra.anger_end = game.world.game_time + 500;
            if let Some(conn) = guest
                && let Some(host) = &mut game.lan
            {
                host.mob_players.insert(id, conn);
            }
        }
        let raw = game
            .persist
            .raw
            .entry(id)
            .or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw {
            fields.insert("HillCooldown".into(), Tag::Int(cooldown));
            if owner != Some(actor) {
                fields.insert("NaturalistAngryActor".into(), Tag::IntArray(actor.to_vec()));
            }
        }
    }
}
