// Goal module for Rhino's calf-protection behavior. The entity tick mixin
// installs/removes the vanilla target and melee goals as a calf enters/leaves
// the 16-block defense radius.
use horizon_core::nbt::Tag;

pub fn stun(game: &mut super::Game<'_>, id: u32) {
    let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id && mob.kind.id == "naturalist:rhino") else { return; };
    mob.extra.counter2 = 60;
    horizon_client::mobs::finalize::set_base(mob, horizon_client::mobs::finalize::Attr::MovementSpeed, 0.0);
    let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
    if let Tag::Compound(fields) = raw { fields.insert("StunTick".into(), Tag::Int(60)); }
}

pub fn tick(game: &mut super::Game<'_>, id: u32) {
    let Some(index) = game.mobs.iter().position(|mob| mob.id == id && mob.kind.id == "naturalist:rhino") else { return; };
    let stun = game.persist.raw.get(&id).and_then(|tag| tag.get(&"StunTick".into()))
        .and_then(horizon_world::datafix::dynamic::as_number).map_or(0, |value| value.int_value());
    if stun > 1 {
        horizon_client::mobs::finalize::set_base(&mut game.mobs[index], horizon_client::mobs::finalize::Attr::MovementSpeed, 0.0);
        let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw { fields.insert("StunTick".into(), Tag::Int(stun - 1)); }
    } else if stun == 1 {
        horizon_client::mobs::finalize::set_base(&mut game.mobs[index], horizon_client::mobs::finalize::Attr::MovementSpeed, 0.2);
        let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw { fields.insert("StunTick".into(), Tag::Int(0)); }
    }
}
