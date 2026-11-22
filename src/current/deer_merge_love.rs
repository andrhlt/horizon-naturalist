fn deer_merge_love(
    game: &mut Game<'_>,
    target: crate::game::commands::Target,
    tag: horizon_core::nbt::Tag,
    original: impl FnOnce(&mut Game<'_>, crate::game::commands::Target, horizon_core::nbt::Tag) -> R<()>,
) -> R<()> {
    use horizon_core::nbt::Tag;
    let love = tag
        .get("InLove")
        .and_then(horizon_world::datafix::dynamic::as_number)
        .map(|n| n.int_value())
        .unwrap_or(0);
    let cause = match tag.get("LoveCause") {
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
    let health_tag = tag.clone();
    let result = original(game, target, tag);
    if result.is_ok() && let crate::game::commands::Target::Mob(id) = target {
        let kind = game.mobs.iter().find(|mob| mob.id == id).map(|mob| mob.kind.id.to_string());
        if let Some(kind) = kind {
            crate::game::naturalist_wolf_health::restore(game, id, &kind, Some(&health_tag));
        }
    }
    if result.is_ok() && let crate::game::commands::Target::Mob(id) = target {
        let kind = game.mobs.iter().find(|mob| mob.id == id).map(|mob| mob.kind.id.to_string());
        if let Some(kind) = kind {
            crate::game::naturalist_wolf_health::restore(game, id, &kind, Some(&health_tag));
        }
    }
    if result.is_ok() && let crate::game::commands::Target::Mob(id) = target {
        let tags = game.world_tables.as_ref().and_then(|t| t.pack_tags.get("entity_type")).unwrap_or(&game.res.mob_data().entity_tags);
        let predators = tags.get("naturalist:deer_predators").cloned().unwrap_or_default();
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) {
            crate::game::naturalist_deer_goals::install(mob, predators);
        }
        crate::game::naturalist_deer_goals::normalize_variant(game, id);
    }
    if result.is_ok()
        && let crate::game::commands::Target::Mob(id) = target
        && let Some(mob) = game
            .mobs
            .iter_mut()
            .find(|mob| mob.id == id && mob.kind.id == "naturalist:deer")
    {
        mob.extra.in_love = love;
        mob.extra.love_cause = cause;
    }
    result
}
