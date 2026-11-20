fn deer_load_love(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    use horizon_core::nbt::Tag;
    let tags = game
        .world_tables
        .as_ref()
        .and_then(|t| t.pack_tags.get("entity_type"))
        .unwrap_or(&game.res.mob_data().entity_tags);
    let predators = tags
        .get("naturalist:deer_predators")
        .cloned()
        .unwrap_or_default();
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
        crate::game::naturalist_deer_goals::install(mob, predators);
    }
    if let Some(variant) = crate::game::naturalist_deer_goals::take_pending_variant(entity) {
        let raw = game
            .persist
            .raw
            .entry(entity)
            .or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw {
            fields.insert("Variant".into(), Tag::String(variant.as_str().into()));
        }
    }
    crate::game::naturalist_deer_goals::normalize_variant(game, entity);
    if kind != "naturalist:deer" {
        return;
    }
    let Some(raw) = game.persist.raw.get(&entity) else {
        return;
    };
    let love = raw
        .get("InLove")
        .and_then(horizon_world::datafix::dynamic::as_number)
        .map(|n| n.int_value())
        .unwrap_or(0);
    let cause = match raw.get("LoveCause") {
        Some(Tag::IntArray(words)) if words.len() == 4 => {
            Some([words[0], words[1], words[2], words[3]])
        }
        Some(Tag::String(value)) => {
            let text = value.to_string_lossy().replace('-', "");
            if text.len() != 32 {
                None
            } else {
                u128::from_str_radix(&text, 16).ok().map(|n| {
                    [
                        (n >> 96) as i32,
                        (n >> 64) as i32,
                        (n >> 32) as i32,
                        n as i32,
                    ]
                })
            }
        }
        _ => None,
    };
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
        mob.extra.in_love = love;
        mob.extra.love_cause = cause;
    }
}
