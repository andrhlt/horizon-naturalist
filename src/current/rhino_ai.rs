// Goal module for Rhino's calf defense, staged charge and shield stun.
use horizon_core::{geometry::Vec3, nbt::Tag};
use horizon_client::mobs::{Damage, DamageKind, Mob, MobEvent, Who, goals::{Cx, Goal, LOOK, MOVE}};

#[derive(Clone, Debug, Default)]
pub struct RhinoCharge {
    target: Option<Who>,
    windup: u8,
    charge: u8,
    cooldown: u8,
}

impl Goal for RhinoCharge {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn name(&self) -> &'static str { "NaturalistRhinoCharge" }
    fn flags(&self) -> u8 { MOVE | LOOK }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if self.cooldown > 0 { self.cooldown -= 1; return false; }
        let Some(target) = mob.ai.target else { return false; };
        if cx.w.target_info(target).is_none_or(|info| !info.alive) { return false; }
        self.target = Some(target);
        // Java's prepare goal counts its 42 tick charge cooldown down before
        // the Rhino starts moving. Keep the visible windup in step with that
        // same interval instead of charging after an arbitrary short pause.
        self.windup = 42;
        self.charge = 0;
        mob.ints[5] = 0;
        true
    }

    fn can_continue(&mut self, _: &mut Mob, cx: &mut Cx) -> bool {
        self.target.and_then(|who| cx.w.target_info(who)).is_some_and(|target| target.alive)
            && (self.windup > 0 || self.charge > 0)
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        mob.nav_stop();
        mob.ints[5] = 0;
        self.target = None;
        self.windup = 0;
        self.charge = 0;
        self.cooldown = 42;
    }

    fn every_tick(&self) -> bool { true }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(who) = self.target else { return; };
        let Some(target) = cx.w.target_info(who) else { return; };
        mob.look_at([target.position.x, target.eye_y, target.position.z]);
        if self.windup > 0 {
            mob.nav_stop();
            mob.ints[5] = 0;
            self.windup -= 1;
            if self.windup == 0 { self.charge = 60; }
            return;
        }

        let delta = Vec3::new(target.position.x - mob.position.x, 0.0,
                              target.position.z - mob.position.z);
        if delta.length_squared() <= 4.0 {
            let from = [mob.position.x, mob.position.y, mob.position.z];
            let amount = mob.attrs.attack_damage;
            match who {
                Who::Player => cx.events.push(cx.w.route_player_event(MobEvent::DamagePlayer {
                    amount, from, kind: "minecraft:mob_attack_no_aggro",
                })),
                Who::Mob(id) => cx.events.push(MobEvent::AttackMob {
                    target: id, amount,
                    source: Damage { kind: DamageKind::MobAttackNoAggro, ..Damage::mob(mob.id, from) },
                }),
            }
            self.charge = 0;
            self.cooldown = 42;
            mob.ints[5] = 0;
            mob.nav_stop();
            return;
        }
        mob.nav_move_to_entity(cx.w, [target.position.x, target.position.y, target.position.z], 0, 2.5);
        self.charge = self.charge.saturating_sub(1);
        mob.ints[5] = i32::from(self.charge > 0);
    }
}

pub fn stun(game: &mut super::Game<'_>, id: u32) {
    let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id && mob.kind.id == "naturalist:rhino") else { return; };
    mob.extra.counter2 = 60;
    mob.ints[5] = 0;
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
