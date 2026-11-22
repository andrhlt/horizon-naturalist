fn rat_owner_join(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    if kind != "naturalist:rat" { return; }
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
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
        horizon_client::mobs::tame::restore_sitting(mob, sitting); mob.extra.owner = owner; mob.extra.tame = owner.is_some(); mob.extra.from_bucket = from_bucket;
    }
}
