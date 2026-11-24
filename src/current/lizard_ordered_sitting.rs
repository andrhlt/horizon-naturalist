fn lizard_ordered_sitting(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    use horizon_core::nbt::Tag;

    if kind != "naturalist:lizard" {
        return;
    }
    let Some(horizon_core::nbt::Tag::Compound(data)) = game.persist.raw.get(&entity) else {
        return;
    };
    let tamed = matches!(data.get(&"Tame".into()), Some(Tag::Byte(value)) if *value != 0)
        || data.contains_key(&"Owner".into());
    if !tamed {
        return;
    }
    let sitting = data
        .get(&"NaturalistOrderedSitting".into())
        .or_else(|| data.get(&"Sitting".into()))
        .is_some_and(|tag| matches!(tag, Tag::Byte(value) if *value != 0));
    let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) else {
        return;
    };
    if mob.extra.sitting != sitting || mob.extra.sitting_pose != sitting {
        horizon_client::mobs::tame::restore_sitting(mob, sitting);
    }
}
