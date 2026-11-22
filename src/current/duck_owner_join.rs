fn duck_owner_join(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    if kind != "naturalist:duck" { return; }
    use mod_api::{Host, prelude::ServicesServerHost};
    if let Some(handle)=game.entity(entity).and_then(|e|game.mob(e)) { handle.remove_goal(game,"follow_parent"); }
    if let Some(mob)=game.mobs.iter_mut().find(|m|m.id==entity) && !mob.ai.goals.has("NaturalistDuckFollowParent") {
        mob.ai.goals.add(4,crate::game::naturalist_duck_follow::DuckFollowParent::default());
    }
    let raw = game.persist.raw.get(&entity);
    let owner = raw.and_then(horizon_client::entity_nbt::owner_uuid);
    let from_bucket = raw.and_then(|tag|tag.get("FromBucket")).and_then(horizon_world::datafix::dynamic::as_number).is_some_and(|n|n.int_value()!=0);
    let dye = raw.and_then(|tag|tag.get("DyeColor")).and_then(horizon_world::datafix::dynamic::as_number).map(|n|n.int_value()).filter(|n|(0..16).contains(n)).map_or(u8::MAX, |n|n as u8);
    let sitting = raw.and_then(|tag|tag.get("Sitting")).and_then(horizon_world::datafix::dynamic::as_number).is_some_and(|n|n.int_value()!=0);
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
        horizon_client::mobs::tame::restore_sitting(mob, sitting); mob.extra.owner = owner; mob.extra.tame = owner.is_some(); mob.extra.from_bucket = from_bucket; mob.extra.color = dye;
    }
}
