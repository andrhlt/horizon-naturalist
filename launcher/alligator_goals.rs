use std::sync::Arc;

use super::super::{
    control::{LookKind, MoveKind},
    nav::{NavKind, naturalist_catfish_state as hunt},
    path::PathType,
    PlayerView,
};
use super::{
    Breed, Cx, FollowParent, Goal, HurtByTarget, JUMP, LOOK, LookAtPlayer, MOVE, MeleeAttack, Mob,
    MobEvent, MobWorld, MoveToBlock, Panic, RandomLookAround, Stroll, TARGET, TargetState, Who,
    goal_clone, reduced, target_continue,
};
use horizon_core::{geometry::Vec3, random::RandomSource};

pub const KIND: &str = "naturalist:alligator";
pub const PREY: &str = "naturalist:alligator_hostiles";
pub const EGG: &str = "naturalist:alligator_egg";

pub(in crate::mobs) fn register(mob: &mut Mob) {
    let kind = std::sync::Arc::make_mut(&mut mob.kind);
    kind.food = ["beef", "porkchop", "chicken", "rabbit"]
        .map(|path| format!("minecraft:{path}"))
        .into();
    kind.flags.breathes_water = true;
    kind.max_head_y_rot = 40.0;
    kind.water_slowdown = 0.98;
    mob.set_malus(PathType::Water, 0.0);
    mob.set_malus(PathType::WaterBorder, 0.0);
    mob.ai.nav.kind = NavKind::Amphibious {
        prefers_shallow: false,
    };
    mob.ai.mov.kind = MoveKind::SmoothSwimming {
        max_turn_x: 85.0,
        max_turn_y: 10.0,
        in_water: 0.4,
        outside: 1.0,
        gravity: false,
    };
    mob.ai.look.kind = LookKind::SmoothSwimming(20.0);
    mob.ai.goals.clear();
    mob.ai.goals.add(1, EggBreed(Breed::new(1.0)));
    mob.ai.goals.add(1, LayEgg(MoveToBlock::new(1.0, 16, 1)));
    mob.ai.goals.add(2, Pursue(MeleeAttack::new(1.2, true)));
    mob.ai.goals.add(3, BabyPanic(Panic::new(1.25)));
    mob.ai.goals.add(4, FollowGator(FollowParent::new(1.2)));
    mob.ai.goals.add(5, Stroll::swimming(1.0, 10));
    mob.ai.goals.add(5, Stroll::new(1.0));
    mob.ai.goals.add(6, LookAtPlayer::new(6.0));
    mob.ai.goals.add(7, RandomLookAround::default());
    mob.ai.goals.add(
        8,
        SniffFlowers {
            base: MoveToBlock::new(1.0, 16, 4),
            waited: 0,
        },
    );
    mob.ai.targets.clear();
    mob.ai.targets.add(
        1,
        HurtByTarget::new()
            .polar_bear()
            .alert_if(|seen| seen.effects.contains(&KIND) && !seen.baby),
    );
    mob.ai.targets.add(1, Target::new(TargetKind::Nest));
    mob.ai.targets.add(2, Target::new(TargetKind::Player));
    mob.ai.targets.add(3, Target::new(TargetKind::Prey));
}

pub(super) fn site(world: &MobWorld<'_>, [x, y, z]: [i32; 3]) -> bool {
    world.has_tag(
        "naturalist:alligator_egg_layable_on",
        world.block_name(x, y, z),
    ) && world.facts.is_air(world.world.block(x, y + 1, z))
}

#[derive(Clone, Debug)]
struct EggBreed(Breed);
impl Goal for EggBreed {
    goal_clone!();
    fn flags(&self) -> u8 {
        self.0.flags()
    }
    fn every_tick(&self) -> bool {
        self.0.every_tick()
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if m.extra.flag || m.extra.in_love <= 0 {
            return false;
        }
        let mut best = 64.0;
        self.0.partner = None;
        for other in cx.w.nearby(m.position, 8.0, |other| {
            other.id != m.id
                && other.alive
                && other.in_love
                && !other.baby
                && !other.panicking
                && other.effects.contains(&KIND)
        }) {
            let distance = m.distance_sqr(other.position);
            if distance <= best {
                self.0.partner = Some(other.id);
                best = distance;
            }
        }
        self.0.partner.is_some()
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        self.0.can_continue(m, cx)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.start(m, cx);
    }
    fn stop(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.stop(m, cx);
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.tick(m, cx);
    }
}

#[derive(Clone, Debug)]
pub(super) struct LayEgg(pub(super) MoveToBlock);
impl Goal for LayEgg {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE | JUMP
    }
    fn every_tick(&self) -> bool {
        true
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if !m.extra.flag {
            return false;
        }
        if self.0.next_start > 0 {
            self.0.next_start -= 1;
            return false;
        }
        self.0.next_start = reduced(200 + m.random.next_int_bound(200).unwrap_or(0));
        self.0.find_nearest(m, cx.w, site)
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        m.extra.flag && self.0.can_continue(cx.w, site)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.start(m, cx.w);
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.tick(m, cx.w);
        if m.fluids.in_water || !self.0.reached {
            return;
        }
        if m.extra.counter > 60 {
            let [x, y, z] = self.0.block;
            let [mx, my, mz] = m.position.components().map(|v| v.floor());
            cx.events.push(MobEvent::Sound(
                "entity.turtle.lay_egg",
                [mx + 0.5, my + 0.5, mz + 0.5],
                0.3,
                0.9 + cx.lr.next_float() * 0.2,
            ));
            let count = m.random.next_int_bound(4).unwrap_or(0) + 1;
            if let Some(base) = cx.w.registry.default_state(EGG)
                && let Some(state) = super::naturalist_insect_goals::state_property(
                    cx.w.registry,
                    base,
                    "eggs",
                    &count.to_string(),
                )
            {
                cx.events.push(MobEvent::SetBlock {
                    pos: [x, y + 1, z],
                    state,
                });
            }
            m.extra.flag = false;
            m.extra.flag2 = false;
            m.extra.counter = 0;
            m.extra.in_love = 600;
        } else {
            m.extra.flag2 = true;
            m.extra.counter += if m.extra.counter == 0 { 2 } else { 1 };
        }
    }
}

#[derive(Clone, Debug)]
struct BabyPanic(Panic);
impl Goal for BabyPanic {
    goal_clone!();
    fn flags(&self) -> u8 {
        self.0.flags()
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        (m.is_baby() || m.fire_ticks > 0) && self.0.can_use(m, cx)
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        self.0.can_continue(m, cx)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.start(m, cx);
    }
    fn stop(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.stop(m, cx);
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.tick(m, cx);
    }
}

#[derive(Clone, Debug)]
struct FollowGator(FollowParent);
impl Goal for FollowGator {
    goal_clone!();
    fn flags(&self) -> u8 {
        self.0.flags()
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if !m.is_baby() {
            return false;
        }
        let mut best = None;
        let area = m.bounds().inflate(Vec3::new(8.0, 4.0, 8.0));
        for seen in cx.w.others_near(area).filter(|s| {
            s.alive
                && !s.baby
                && s.effects.contains(&KIND)
                && area.intersects(s.bounds)
        }) {
            let distance = m.distance_sqr(seen.position);
            if best.is_none_or(|(_, best)| distance <= best) {
                best = Some((seen.id, distance));
            }
        }
        self.0.parent = best.filter(|(_, d)| *d >= 9.0).map(|(id, _)| id);
        self.0.parent.is_some()
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        self.0.can_continue(m, cx)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.start(m, cx);
    }
    fn stop(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.stop(m, cx);
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.tick(m, cx);
    }
}

#[derive(Clone, Debug)]
struct SniffFlowers {
    base: MoveToBlock,
    waited: i32,
}
impl SniffFlowers {
    fn flower(w: &MobWorld<'_>, [x, y, z]: [i32; 3]) -> bool {
        w.has_tag("minecraft:flowers", w.block_name(x, y, z))
    }

    fn approach(&self, m: &mut Mob, w: &MobWorld<'_>) {
        let [x, y, z] = self.base.block.map(f64::from);
        m.nav_move_to(w, [x + 0.5, y, z + 0.5], self.base.speed);
    }
}
impl Goal for SniffFlowers {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE | JUMP | LOOK
    }
    fn every_tick(&self) -> bool {
        true
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if !m.is_baby() {
            return false;
        }
        if self.base.next_start > 0 {
            self.base.next_start -= 1;
            return false;
        }
        self.base.next_start = reduced(200 + m.random.next_int_bound(200).unwrap_or(0));
        let [x, y, z] = m.position.components().map(|v| v.floor() as i32);
        if !cx
            .w
            .world
            .may_hold([x - 16, y - 5, z - 16], [x + 16, y + 3, z + 16], |state| {
                cx.w.has_tag("minecraft:flowers", &cx.w.facts.get(state).name)
            })
        {
            return false;
        }
        let mut sites = Vec::new();
        for dy in [0, 1, -1, 2, -2, 3, -3, 4, -4] {
            for dx in -16..=16 {
                for dz in -16..=16 {
                    let pos = [x + dx, y + dy - 1, z + dz];
                    let center = Vec3::new(
                        f64::from(pos[0]) + 0.5,
                        f64::from(pos[1]) + 0.5,
                        f64::from(pos[2]) + 0.5,
                    );
                    if m.within_home_at(center) && Self::flower(cx.w, pos) {
                        sites.push(pos);
                    }
                }
            }
        }
        if sites.is_empty() {
            return false;
        }
        self.base.block = sites[m.random.next_int_bound(sites.len() as i32).unwrap_or(0) as usize];
        true
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        m.is_baby() && self.waited < 100 && self.base.can_continue(cx.w, Self::flower)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.base.start(m, cx.w);
        self.approach(m, cx.w);
        self.waited = 0;
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        if self.base.reached {
            m.nav_stop();
            if self.waited % 20 == 0 && !m.silent {
                cx.events.push(MobEvent::Sound(
                    "entity.fox.sniff",
                    m.position.components(),
                    1.0,
                    1.0,
                ));
            }
            self.waited += 1;
        }

        let [x, y, z] = self.base.block.map(f64::from);
        let center = [x + 0.5, y + 0.5, z + 0.5];
        m.look_at_speed(center, 30.0, m.max_head_x_rot());
        self.base.reached = m.distance_sqr(Vec3::new(center[0], center[1], center[2])) < 4.0;
        if self.base.reached {
            self.base.try_ticks -= 1;
        } else {
            self.base.try_ticks += 1;
            if self.base.try_ticks % 100 == 0 {
                self.approach(m, cx.w);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct NestActors {
    candidates: Arc<[(Option<u32>, PlayerView)]>,
    pending: Option<Option<u32>>,
    melee_events: Vec<MobEvent>,
}

pub fn pursuing_swimmer(mob: &Mob) -> bool {
    mob.kind.id == KIND && !mob.is_baby() && !mob.no_ai && mob.extra.leash.is_none()
        && mob.ai.target == Some(Who::Player) && mob.ai.targets.is_running("AlligatorPlayerTarget")
}

pub fn set_nest_actors(mob: &mut Mob, candidates: Arc<[(Option<u32>, PlayerView)]>) {
    *mob.ai.nav.__horizon_mixins_naturalist_alligator_nest_actors_mut() = if candidates.is_empty() {
        None
    } else {
        Some(Box::new(NestActors { candidates, pending: None, melee_events: Vec::new() }))
    };
}

pub fn take_nest_actor(mob: &mut Mob) -> Option<(Option<u32>, Vec<MobEvent>)> {
    let actors = mob.ai.nav.__horizon_mixins_naturalist_alligator_nest_actors_mut().take()?;
    Some((actors.pending?, actors.melee_events))
}

fn nest_handoff_view<'w>(mob: &Mob, world: &MobWorld<'w>) -> Option<MobWorld<'w>> {
    let actors = mob.ai.nav.__horizon_mixins_naturalist_alligator_nest_actors().as_ref()?;
    let pending = actors.pending?;
    let (_, intruder) = actors.candidates.iter().find(|(connection, _)| *connection == pending)?;
    let mut world = world.clone();
    world.player = Some(intruder.clone());
    Some(world)
}

#[derive(Clone, Debug)]
struct Pursue(MeleeAttack);
impl Goal for Pursue {
    goal_clone!();
    fn name(&self) -> &'static str { self.0.name() }
    fn flags(&self) -> u8 { self.0.flags() }
    fn every_tick(&self) -> bool { self.0.every_tick() }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if let Some(world) = nest_handoff_view(mob, cx.w) {
            self.0.can_use(mob, &mut Cx { w: &world, lr: cx.lr, events: cx.events })
        } else {
            self.0.can_use(mob, cx)
        }
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if let Some(world) = nest_handoff_view(mob, cx.w) {
            self.0.can_continue(mob, &mut Cx { w: &world, lr: cx.lr, events: cx.events })
        } else {
            self.0.can_continue(mob, cx)
        }
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.start(mob, cx); }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.stop(mob, cx); }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.tick_pursuit(mob, cx);
    }
}
impl Pursue {
    fn tick_pursuit(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(world) = nest_handoff_view(mob, cx.w) else {
            self.0.tick(mob, cx);
            return;
        };
        // The native attack's Alligator reach and visibility hooks already match Java.
        // Keep its events with the new actor while this turn still belongs to the swimmer.
        let mut events = Vec::new();
        self.0.tick(mob, &mut Cx { w: &world, lr: cx.lr, events: &mut events });
        if let Some(actors) = mob.ai.nav.__horizon_mixins_naturalist_alligator_nest_actors_mut() {
            actors.melee_events = events;
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum TargetKind {
    Nest,
    Player,
    Prey,
}

#[derive(Clone, Debug)]
struct Target {
    kind: TargetKind,
    found: Option<Who>,
    alternate_actor: Option<Option<u32>>,
    state: TargetState,
}
impl Target {
    fn new(kind: TargetKind) -> Self {
        Self {
            kind,
            found: None,
            alternate_actor: None,
            state: TargetState {
                unseen_memory: 60,
                ..Default::default()
            },
        }
    }
}

pub fn near_eggs(world: &MobWorld<'_>, pos: Vec3) -> bool {
    let [x, y, z] = pos.components().map(|v| v.floor() as i32);
    (-2..=2).any(|dx| {
        (-2..=2).any(|dy| (-2..=2).any(|dz| world.block_name(x + dx, y + dy, z + dz) == EGG))
    })
}

impl Goal for Target {
    goal_clone!();
    fn name(&self) -> &'static str {
        match self.kind {
            TargetKind::Nest => "AlligatorNestTarget",
            TargetKind::Player => "AlligatorPlayerTarget",
            TargetKind::Prey => "AlligatorPreyTarget",
        }
    }
    fn flags(&self) -> u8 {
        TARGET
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if m.random.next_int_bound(reduced(10)).unwrap_or(1) != 0 || m.is_baby() {
            return false;
        }
        let world = cx.w;
        let range = m.attrs.follow_range;
        self.found = None;
        self.alternate_actor = None;
        if matches!(self.kind, TargetKind::Player) {
            if let Some(player) = &world.player {
                if (!world.daylight
                    || m.extra.flag
                    || m.extra.flag2
                    || world.fluids(player.bounds, player.eye[1]).in_water)
                    && m.can_target_player(world, range, true)
                {
                    self.found = Some(Who::Player);
                }
            }
            return self.found.is_some();
        }
        if matches!(self.kind, TargetKind::Prey) && hunt::cooldown(&m.ai.nav) > 0 {
            return false;
        }
        let pos = Vec3::new(m.position.x, m.eye_y(), m.position.z);
        let mut nearest = range * range;
        let mut candidates = Vec::new();
        for seen in world.nearby(pos, range, |s| s.id != m.id && !s.playing_dead) {
            let wanted = match self.kind {
                TargetKind::Nest => {
                    !seen.effects.contains(&KIND) && near_eggs(world, seen.position)
                }
                TargetKind::Prey => seen.effects.contains(&PREY),
                TargetKind::Player => false,
            };
            let distance = m.distance_sqr(seen.position);
            if wanted && distance <= nearest {
                candidates.push((Who::Mob(seen.id), distance));
            }
        }
        for (who, distance) in candidates {
            if distance <= nearest && m.sees(world, who) && !m.spares_target(world, who) {
                self.found = Some(who);
                nearest = distance;
            }
        }
        if matches!(self.kind, TargetKind::Nest) {
            if let Some(player) = &world.player
                && near_eggs(world, player.position)
                && m.distance_sqr(player.position) < nearest
                && m.can_target_player(world, range, true)
            {
                self.found = Some(Who::Player);
                nearest = m.distance_sqr(player.position);
            }
            let actors = m.ai.nav.__horizon_mixins_naturalist_alligator_nest_actors();
            if let Some(actors) = actors
                && world.difficulty.base != 0
                && !m.spares_players(world)
            {
                for (connection, player) in actors.candidates.iter() {
                    if world.player.as_ref().is_some_and(|current| current.uuid == player.uuid) {
                        continue;
                    }
                    let distance = m.distance_sqr(player.position);
                    let reach = (range * player.visibility_percent(Some(KIND), world.items)).max(2.0);
                    if distance < nearest && distance <= reach * reach && !player.allies.contains(&m.id)
                        && near_eggs(world, player.position)
                        && world.line_of_sight_for(
                            Some(m.position.y), pos,
                            Vec3::new(player.position.x, player.eye[1], player.position.z),
                        )
                    {
                        nearest = distance;
                        self.found = Some(Who::Player);
                        self.alternate_actor = Some(*connection);
                    }
                }
            }
        }
        self.found.is_some()
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        target_continue(&mut self.state, true, m, cx)
    }
    fn start(&mut self, m: &mut Mob, _: &mut Cx) {
        self.state = TargetState {
            unseen_memory: 60,
            ..Default::default()
        };
        m.ai.target = self.found;
        if let Some(actors) = m.ai.nav.__horizon_mixins_naturalist_alligator_nest_actors_mut() {
            actors.pending = self.alternate_actor.take();
        }
    }
    fn stop(&mut self, m: &mut Mob, _: &mut Cx) {
        m.ai.target = None;
    }
}
