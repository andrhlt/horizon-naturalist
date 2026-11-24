fn rat_owner_join(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    if kind != "naturalist:rat" { return; }
    use mod_api::Host;
    let raw = game.persist.raw.get(&entity);
    let owner = raw.and_then(horizon_client::entity_nbt::owner_uuid);
    let from_bucket = raw.and_then(|tag|tag.get("FromBucket")).and_then(horizon_world::datafix::dynamic::as_number).is_some_and(|n|n.int_value()!=0);
    let sitting = raw.and_then(|tag|tag.get("Sitting")).and_then(horizon_world::datafix::dynamic::as_number).is_some_and(|n|n.int_value()!=0);
    if owner.is_some() {
        let raw = game.persist.raw.entry(entity).or_insert_with(|| horizon_core::nbt::Tag::Compound(Default::default()));
        if let horizon_core::nbt::Tag::Compound(fields) = raw {
            fields.entry("FollowingOwner".into()).or_insert(horizon_core::nbt::Tag::Byte(1));
            fields.entry("Sitting".into()).or_insert(horizon_core::nbt::Tag::Byte(0));
        }
    }
    let baby = game.mobs.iter().find(|mob| mob.id == entity).is_some_and(|mob| mob.extra.age < 0);
    let inherited_owner = if baby && owner.is_none() {
        game.mobs.iter()
            .filter(|mob| mob.id != entity && mob.kind.id == "naturalist:rat" && mob.extra.tame && mob.extra.owner.is_some())
            .filter(|mob| mob.distance_sqr(game.mobs.iter().find(|child| child.id == entity).map_or(mob.position, |child| child.position)) <= 16.0)
            .find_map(|mob| mob.extra.owner)
    } else {
        None
    };
    let owner = owner.or(inherited_owner);
    if let Some(entity_ref) = game.entity(entity)
        && let Some(handle) = game.mob(entity_ref)
    {
        handle.remove_goal(game, "breed");
        handle.remove_goal(game, "Breed");
    }
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
        horizon_client::mobs::tame::restore_sitting(mob, sitting);
        mob.extra.owner = owner;
        mob.extra.tame = owner.is_some();
        mob.extra.from_bucket = from_bucket;
        crate::game::naturalist_rat_breed_goal::install(mob);
    }
    if inherited_owner.is_some() {
        let raw = game.persist.raw.entry(entity).or_insert_with(|| horizon_core::nbt::Tag::Compound(Default::default()));
        if let horizon_core::nbt::Tag::Compound(fields) = raw {
            fields.insert("Owner".into(), horizon_core::nbt::Tag::IntArray(owner.unwrap().to_vec()));
            fields.insert("FollowingOwner".into(), horizon_core::nbt::Tag::Byte(1));
            fields.insert("Sitting".into(), horizon_core::nbt::Tag::Byte(0));
        }
    }
}
