// Goal module for Rhino's calf defense, staged charge and shield stun.
use horizon_core::{geometry::Vec3, nbt::Tag};
use horizon_core::random::RandomSource;
use horizon_client::mobs::{Damage, DamageKind, Mob, MobEvent, Who, goals::{Cx, Goal, LOOK, MOVE}};
use std::collections::HashSet;

#[derive(Clone, Debug, Default)]
pub struct RhinoCharge {
    target: Option<Who>,
    windup: u8,
    charge: u8,
}

#[derive(Clone, Debug)]
pub struct LargeCatPreyTarget {
    prey: HashSet<String>,
    found: Option<Who>,
    state: horizon_client::mobs::goals::TargetState,
}

#[derive(Clone, Debug, Default)]
struct LargeCatFollowOwner {
    recalc: i32,
}

impl LargeCatFollowOwner {
    fn owner(mob: &Mob, cx: &Cx) -> Option<(Vec3, f64)> {
        let (uuid, _) = cx.w.player_identity?;
        if !mob.extra.tame || !horizon_client::mobs::tame::owned_by(mob.extra.owner, uuid) {
            return None;
        }
        let player = cx.w.player.as_ref()?;
        (horizon_client::mobs::tame::owned_by(mob.extra.owner, player.uuid)
            && player.alive && !player.spectator)
            .then_some((player.position, player.eye[1]))
    }
}

// Lion/Tiger Java PetFollowOwnerGoal: only pets ordered to follow move to their
// owner, starting at 7 blocks, stopping at 2, and refreshing a 1.3-speed path.
impl Goal for LargeCatFollowOwner {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn name(&self) -> &'static str { "NaturalistLargeCatFollowOwner" }
    fn flags(&self) -> u8 { MOVE | LOOK }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.extra.ints[4] != 0 && !mob.extra.sitting
            && Self::owner(mob, cx).is_some_and(|(position, _)| mob.distance_sqr(position) >= 49.0)
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.extra.ints[4] != 0 && !mob.extra.sitting && !mob.ai.nav.is_done()
            && Self::owner(mob, cx).is_some_and(|(position, _)| mob.distance_sqr(position) > 4.0)
    }

    fn start(&mut self, _: &mut Mob, _: &mut Cx) { self.recalc = 0; }
    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) { mob.nav_stop(); }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some((position, eye_y)) = Self::owner(mob, cx) else { return; };
        if mob.distance_sqr(position) < 144.0 {
            mob.look_at_speed([position.x, eye_y, position.z], 10.0, mob.max_head_x_rot());
        }
        self.recalc -= 1;
        if self.recalc <= 0 {
            self.recalc = horizon_client::mobs::goals::reduced(10);
            mob.nav_move_to_entity(cx.w, [position.x, position.y, position.z], 0, 1.3);
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TigerStalk {
    time_to_recalc_path: i32,
}

// Pinned Tiger.TigerStalkGoal: untamed adults start above 12 blocks, continue
// above 6, and refresh a 0.7-speed path every adjusted 10 ticks.
impl Goal for TigerStalk {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn name(&self) -> &'static str { "NaturalistTigerStalk" }
    fn flags(&self) -> u8 { MOVE | LOOK }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        !mob.is_baby()
            && !mob.extra.tame
            && mob.pose() != mod_api::Pose::Sleeping
            && mob.target_info(cx.w).is_some_and(|target| {
                target.alive && mob.distance_sqr(target.position) > 144.0
            })
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.pose() != mod_api::Pose::Sleeping
            && mob.target_info(cx.w).is_some_and(|target| {
                target.alive && mob.distance_sqr(target.position) > 36.0
            })
    }

    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        mob.extra.pose = horizon_client::mobs::sync::CROUCHING;
        self.time_to_recalc_path = 0;
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        mob.extra.pose = horizon_client::mobs::sync::STANDING;
        mob.nav_stop();
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(target) = mob.target_info(cx.w) else { return; };
        mob.look_at_speed(
            [target.position.x, target.eye_y, target.position.z],
            10.0,
            mob.kind.max_head_x_rot,
        );
        self.time_to_recalc_path -= 1;
        if self.time_to_recalc_path <= 0 {
            self.time_to_recalc_path = horizon_client::mobs::goals::reduced(10);
            mob.nav_move_to_entity(cx.w, [target.position.x, target.position.y, target.position.z], 0, 0.7);
        }
    }
}

impl LargeCatPreyTarget {
    pub fn new(prey: HashSet<String>) -> Self {
        Self { prey, found: None, state: Default::default() }
    }
}

impl Goal for LargeCatPreyTarget {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn name(&self) -> &'static str { "NaturalistLargeCatPreyTarget" }
    fn flags(&self) -> u8 { horizon_client::mobs::goals::TARGET }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if horizon_client::mobs::animal_brain::hunting_cooldown(mob)
            .is_some_and(|ticks| ticks > 0)
        {
            return false;
        }
        if mob.random.next_int_bound(10).unwrap_or(1) != 0 { return false; }
        let origin = Vec3::new(mob.position.x, mob.eye_y(), mob.position.z);
        let range = mob.attrs.follow_range;
        let candidates: Vec<u32> = cx.w.nearby(origin, range, |seen| {
            seen.id != mob.id && !seen.playing_dead && self.prey.contains(seen.kind.id.as_str())
        }).map(|seen| seen.id).collect();
        self.found = candidates.into_iter().find_map(|id| {
            let who = Who::Mob(id);
            let target = cx.w.target_info(who)?;
            (target.alive && target.can_be_seen_as_enemy
                && mob.distance_sqr(target.position) <= cx.w.reach_sqr(id, mob, range)
                && mob.sees(cx.w, who)).then_some(who)
        });
        self.found.is_some()
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        horizon_client::mobs::goals::target_continue(&mut self.state, true, mob, cx)
    }

    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        mob.set_ai_target(cx.w, self.found);
    }

    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) {
        mob.set_ai_target(cx.w, None);
        self.found = None;
        self.state = Default::default();
    }
}

pub fn install_large_cat(game: &mut horizon_client::game::Game<'_>, entity: u32, kind: &str) {
    let tag = match kind {
        "naturalist:lion" => "naturalist:lion_hostiles",
        "naturalist:tiger" => "naturalist:tiger_hostiles",
        _ => return,
    };
    let tags = game.world_tables.as_ref()
        .and_then(|tables| tables.pack_tags.get("entity_type"))
        .unwrap_or(&game.res.mob_data().entity_tags);
    let prey = tags.get(tag).cloned().unwrap_or_default();
    let following = game.persist.raw.get(&entity)
        .and_then(|raw| raw.get("FollowingOwner"))
        .and_then(horizon_world::datafix::dynamic::as_number)
        .is_none_or(|value| value.int_value() != 0);
    let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) else { return; };
    mob.extra.ints[4] = i32::from(following);
    if mob.ai.targets.has("NaturalistLargeCatPreyTarget") { return; }
    mob.ai.goals.add(5, LargeCatFollowOwner::default());
    mob.ai.targets.add(1, LargeCatPreyTarget::new(prey));
    mob.ai.targets.add(2, horizon_client::mobs::goals::HurtByTarget::new());
    if kind == "naturalist:tiger" {
        mob.ai.goals.add(1, TigerStalk::default());
        mob.ai.goals.add(2, horizon_client::mobs::goals::MeleeAttack::new(1.2, true));
    } else {
        mob.ai.goals.add(1, horizon_client::mobs::goals::MeleeAttack::new(1.2, true));
    }
}

impl Goal for RhinoCharge {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn name(&self) -> &'static str { "NaturalistRhinoCharge" }
    fn flags(&self) -> u8 { MOVE | LOOK }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if mob.extra.counter2 > 0 { return false; }
        let Some(target) = mob.ai.target else { return false; };
        if cx.w.target_info(target).is_none_or(|info| !info.alive) { return false; }
        self.target = Some(target);
        // Java's prepare goal counts its 42 tick charge cooldown down before
        // the Rhino starts moving. Keep the visible windup in step with that
        // same interval instead of charging after an arbitrary short pause.
        self.windup = 42;
        self.charge = 0;
        mob.extra.ints[4] = 0;
        true
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.extra.counter2 <= 0
            && self.target.and_then(|who| cx.w.target_info(who)).is_some_and(|target| target.alive)
            && (self.windup > 0 || self.charge > 0)
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        mob.nav_stop();
        mob.extra.ints[4] = 0;
        self.target = None;
        self.windup = 0;
        self.charge = 0;
    }

    fn every_tick(&self) -> bool { true }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(who) = self.target else { return; };
        let Some(target) = cx.w.target_info(who) else { return; };
        mob.look_at([target.position.x, target.eye_y, target.position.z]);
        if self.windup > 0 {
            mob.nav_stop();
            mob.extra.ints[4] = 0;
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
            mob.extra.ints[4] = 0;
            mob.nav_stop();
            return;
        }
        mob.nav_move_to_entity(cx.w, [target.position.x, target.position.y, target.position.z], 0, 2.5);
        self.charge = self.charge.saturating_sub(1);
        mob.extra.ints[4] = i32::from(self.charge > 0);
    }
}

pub fn stun(game: &mut horizon_client::game::Game<'_>, id: u32) {
    let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id && mob.kind.id == "naturalist:rhino") else { return; };
    mob.extra.counter2 = 60;
    mob.extra.ints[4] = 0;
    mob.nav_stop();
    horizon_client::mobs::finalize::set_base(mob, horizon_client::mobs::finalize::Attr::MovementSpeed, 0.0);
    let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
    if let Tag::Compound(fields) = raw { fields.insert("StunTick".into(), Tag::Int(60)); }
}

pub fn tick(game: &mut horizon_client::game::Game<'_>, id: u32) {
    let Some(index) = game.mobs.iter().position(|mob| mob.id == id && mob.kind.id == "naturalist:rhino") else { return; };
    let stun = game.persist.raw.get(&id).and_then(|tag| tag.get("StunTick"))
        .and_then(horizon_world::datafix::dynamic::as_number).map_or(0, |value| value.int_value());
    game.mobs[index].extra.counter2 = stun.saturating_sub(1).max(0);
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

pub fn break_charge_blocks(game: &mut horizon_client::game::Game<'_>, index: usize) {
    use mod_api::BlockUpdateFlags;

    if game.mobs[index].extra.ints[4] == 0
        || !game.ui.rules.bool_at(horizon_client::gamerules::vanilla::MOB_GRIEFING)
    {
        return;
    }

    let mob = &game.mobs[index];
    let yaw = f64::from(mob.yaw.to_radians());
    let front_x = mob.position.x - yaw.sin() * 1.65;
    let front_z = mob.position.z + yaw.cos() * 1.65;
    let center = [front_x.floor() as i32, mob.position.y.floor() as i32, front_z.floor() as i32];
    let tags = game
        .world_tables
        .as_ref()
        .and_then(|tables| tables.pack_tags.get("block"))
        .unwrap_or(&game.res.items().block_tags);
    let mut breakable = Vec::new();
    for x in center[0] - 1..=center[0] + 1 {
        for y in center[1]..=center[1] + 2 {
            for z in center[2] - 1..=center[2] + 1 {
                let state = game.world.block(x, y, z);
                let name = &game.res.facts().get(state).name;
                if tags
                    .get("naturalist:rhino_charge_breakable")
                    .is_some_and(|blocks| blocks.contains(name.as_ref()))
                {
                    breakable.push([x, y, z]);
                }
            }
        }
    }
    if breakable.is_empty() { return; }
    let Some(level) = game.level(game.dim.id()) else { return; };
    let Some(air) = game.pack.assets.registry.default_state("minecraft:air") else { return; };
    for pos in breakable {
        let _ = game.level_set_block(
            level,
            pos,
            air,
            BlockUpdateFlags::NEIGHBORS | BlockUpdateFlags::CLIENTS,
        );
    }
}
