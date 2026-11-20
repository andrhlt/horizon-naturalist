use super::{
    Breed, Cx, Float, FollowParent, Goal, JUMP, LOOK, LookAtPlayer, MOVE, Mob, MobEvent, MobWorld,
    Panic, RandomLookAround, Stroll, Tempt, goal_clone,
};
use crate::mobs::{Who, nav::Navigation};
use horizon_core::{geometry::Vec3, random::RandomSource};
use horizon_world::registry::BlockStateId;
use std::sync::Arc;

pub const MOLE: &str = "naturalist:mole";
pub const TRAIL: &str = "naturalist:dirt_trail";
pub const FOOD: &str = "minecraft:spider_eye";
pub const UNDEAD: &str = "minecraft:undead";
pub const DIGGABLE: &str = "naturalist:mole_diggable";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum Phase {
    #[default]
    Unrolled = 0,
    Digging = 1,
    Underground = 2,
    Peeking = 3,
    Unrolling = 4,
}

impl Phase {
    pub fn from_saved(value: i32) -> Self {
        match value {
            1 => Self::Digging,
            2 => Self::Underground,
            3 => Self::Peeking,
            4 => Self::Unrolling,
            _ => Self::Unrolled,
        }
    }

    pub fn hides(self) -> bool {
        matches!(self, Self::Digging | Self::Peeking | Self::Unrolling)
    }
}

#[derive(Clone, Debug, Default)]
pub struct MoleState {
    pub phase: Phase,
    phase_ticks: u8,
    calm_ticks: u8,
    peek_ticks: u16,
    threatened: bool,
    peek_saw_threat: bool,
    attacker: Option<Who>,
    attacker_ticks: u16,
    lockout_ticks: u8,
    hold_ticks: u8,
    grace_ticks: u8,
    last_mound: Option<[i32; 3]>,
    pub pending_mound: Option<[i32; 3]>,
    pub wolf_dig: Option<WolfDigTick>,
    pub wolf_digging: bool,
}

pub fn state(nav: &Navigation) -> &MoleState {
    nav.__horizon_mixins_naturalist_mole_state()
}

pub fn state_mut(nav: &mut Navigation) -> &mut MoleState {
    nav.__horizon_mixins_naturalist_mole_state_mut()
}

pub fn phase(mob: &Mob) -> Phase {
    state(&mob.ai.nav).phase
}

fn apply_phase(mob: &mut Mob, phase: Phase) {
    let peek_delay = (phase == Phase::Underground)
        .then(|| 100 + mob.random.next_int_bound(300).unwrap_or(0) as u16);
    let state = state_mut(&mut mob.ai.nav);
    state.phase = phase;
    match phase {
        Phase::Unrolled => {}
        Phase::Digging => state.phase_ticks = 20,
        Phase::Underground => {
            state.calm_ticks = 0;
            state.peek_ticks = peek_delay.unwrap_or(100);
            state.last_mound = None;
            state.grace_ticks = 60;
        }
        Phase::Peeking => {
            state.phase_ticks = 57;
            state.peek_saw_threat = false;
        }
        Phase::Unrolling => state.phase_ticks = 25,
    }
    let kind = Arc::make_mut(&mut mob.kind);
    kind.step_height = if phase == Phase::Unrolled { 0.6 } else { 1.0 };
    Arc::make_mut(&mut kind.attributes)
        .insert("minecraft:step_height".to_owned(), kind.step_height);
}

pub fn load_phase(mob: &mut Mob, saved: i32) {
    apply_phase(mob, Phase::from_saved(saved));
}

fn set_phase(mob: &mut Mob, phase: Phase) {
    apply_phase(mob, phase);
    if phase.hides() {
        mob.position.x = mob.position.x.floor() + 0.5;
        mob.position.z = mob.position.z.floor() + 0.5;
        freeze(mob);
    }
    if phase == Phase::Underground {
        mark_mound(mob);
    }
}

fn mark_mound(mob: &mut Mob) {
    if !mob.on_ground {
        return;
    }
    let pos = mob
        .position
        .components()
        .map(|coordinate| coordinate.floor() as i32);
    let state = state_mut(&mut mob.ai.nav);
    if state.last_mound != Some(pos) {
        state.last_mound = Some(pos);
        state.pending_mound = Some(pos);
    }
}

fn freeze(mob: &mut Mob) {
    mob.nav_stop();
    mob.ai.jumping = false;
    mob.ai.xxa = 0.0;
    mob.ai.zza = 0.0;
    mob.ai.jump = false;
    mob.ai
        .mov
        .set_wanted(mob.position.x, mob.position.y, mob.position.z, 0.0);
    mob.velocity.x = 0.0;
    mob.velocity.z = 0.0;
}

fn forced_out(mob: &Mob) -> bool {
    mob.fire_ticks > 0
        || mob.fluids.in_water
        || mob.extra.leash.is_some()
        || mob.extra.mount.is_some()
}

fn try_dig(mob: &mut Mob) {
    if phase(mob) == Phase::Unrolled
        && state(&mob.ai.nav).lockout_ticks == 0
        && mob.on_ground
        && mob.ticks_frozen == 0
        && !forced_out(mob)
    {
        set_phase(mob, Phase::Digging);
    }
}

pub fn can_be_dug_out(mob: &Mob) -> bool {
    let state = state(&mob.ai.nav);
    state.phase == Phase::Underground && state.grace_ticks == 0
}

pub fn hold_for_digging(mob: &mut Mob) {
    state_mut(&mut mob.ai.nav).hold_ticks = 3;
}

pub fn pop_out(mob: &mut Mob, attacker: Who) {
    let state = state_mut(&mut mob.ai.nav);
    state.attacker = Some(attacker);
    state.attacker_ticks = 1200;
    state.lockout_ticks = 60;
    state.hold_ticks = 0;
    if !matches!(state.phase, Phase::Unrolled | Phase::Unrolling) {
        set_phase(mob, Phase::Unrolling);
    }
}

pub fn after_hit(mob: &mut Mob, attacker: Who) {
    if phase(mob) == Phase::Peeking {
        pop_out(mob, attacker);
    } else {
        let state = state_mut(&mut mob.ai.nav);
        state.attacker = Some(attacker);
        state.attacker_ticks = 1200;
        try_dig(mob);
    }
}

pub fn step(mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) {
    let forced = forced_out(mob);
    let bounds = mob.bounds().inflate(Vec3::new(7.0, 2.0, 7.0));
    let timers = state_mut(&mut mob.ai.nav);
    timers.attacker_ticks = timers.attacker_ticks.saturating_sub(1);
    timers.lockout_ticks = timers.lockout_ticks.saturating_sub(1);
    timers.grace_ticks = timers.grace_ticks.saturating_sub(1);
    if timers.hold_ticks != 0 {
        timers.hold_ticks -= 1;
        freeze(mob);
        return;
    }
    if mob.age % 4 == 0 {
        let remembered = timers.attacker.filter(|_| timers.attacker_ticks > 0);
        timers.threatened = world.player.as_ref().is_some_and(|player| {
            player.alive
                && !player.spectator
                && bounds.intersects(player.bounds)
                && (player.sprinting || player.passenger || remembered == Some(Who::Player))
        }) || world.others_near(bounds).any(|other| {
            other.id != mob.id
                && other.alive
                && bounds.intersects(other.bounds)
                && (other.effects.contains(&UNDEAD) || remembered == Some(Who::Mob(other.id)))
        });
    }
    let old_phase = timers.phase;
    if old_phase == Phase::Unrolled {
        if timers.threatened {
            try_dig(mob);
            if phase(mob) == Phase::Digging && !mob.silent {
                events.push(MobEvent::Sound(
                    "naturalist:entity.mole.dig_down",
                    mob.position.components(),
                    0.7,
                    1.0,
                ));
            }
        }
        return;
    }
    let next = if forced && old_phase != Phase::Unrolling {
        Some(Phase::Unrolling)
    } else {
        match old_phase {
            Phase::Unrolled => None,
            Phase::Digging => {
                timers.phase_ticks = timers.phase_ticks.saturating_sub(1);
                (timers.phase_ticks == 0).then_some(Phase::Underground)
            }
            Phase::Underground => {
                if timers.threatened {
                    timers.calm_ticks = 0;
                    timers.peek_ticks = timers.peek_ticks.saturating_sub(1);
                    (timers.peek_ticks == 0).then_some(Phase::Peeking)
                } else {
                    timers.calm_ticks += 1;
                    (timers.calm_ticks >= 60).then_some(Phase::Peeking)
                }
            }
            Phase::Peeking => {
                timers.peek_saw_threat |= timers.threatened;
                timers.phase_ticks = timers.phase_ticks.saturating_sub(1);
                (timers.phase_ticks == 0).then_some(if timers.peek_saw_threat {
                    Phase::Digging
                } else {
                    Phase::Unrolling
                })
            }
            Phase::Unrolling if !forced && timers.threatened && timers.lockout_ticks == 0 => {
                Some(Phase::Underground)
            }
            Phase::Unrolling => {
                timers.phase_ticks = timers.phase_ticks.saturating_sub(1);
                (timers.phase_ticks == 0).then_some(Phase::Unrolled)
            }
        }
    };
    if let Some(next) = next {
        set_phase(mob, next);
        let sound = match next {
            Phase::Digging => Some("naturalist:entity.mole.dig_down"),
            Phase::Unrolling => Some("naturalist:entity.mole.dig_up"),
            _ => None,
        };
        if let Some(sound) = sound.filter(|_| !mob.silent) {
            events.push(MobEvent::Sound(sound, mob.position.components(), 0.7, 1.0));
        }
    }
    if old_phase == Phase::Underground
        && mob.velocity.x * mob.velocity.x + mob.velocity.z * mob.velocity.z >= 1e-4
    {
        mark_mound(mob);
    }
    if old_phase == Phase::Peeking && state(&mob.ai.nav).phase_ticks == 44 && !mob.silent {
        events.push(MobEvent::Sound(
            "naturalist:entity.mole.peek",
            mob.position.components(),
            0.7,
            1.0,
        ));
    }
}

#[derive(Clone, Debug)]
struct Hide;
impl Goal for Hide {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE | LOOK | JUMP
    }
    fn can_use(&mut self, mob: &mut Mob, _: &mut Cx) -> bool {
        phase(mob).hides()
    }
    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        freeze(mob);
    }
}

fn scurry(mob: &Mob, _: &MobWorld<'_>) -> bool {
    let state = state(&mob.ai.nav);
    state.phase == Phase::Underground && state.hold_ticks == 0
}

fn scurry_position(mob: &mut Mob, cx: &mut Cx) -> Option<[f64; 3]> {
    mob.default_random_pos(cx.w, 5, 4)
}

#[derive(Clone, Debug)]
struct PhaseGoal<G> {
    goal: G,
    allows: fn(&Mob) -> bool,
    continue_checked: bool,
}

impl<G: Goal + Clone + 'static> Goal for PhaseGoal<G> {
    goal_clone!();
    fn flags(&self) -> u8 {
        self.goal.flags()
    }
    fn every_tick(&self) -> bool {
        self.goal.every_tick()
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        (self.allows)(mob) && self.goal.can_use(mob, cx)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        (!self.continue_checked || (self.allows)(mob)) && self.goal.can_continue(mob, cx)
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.goal.start(mob, cx);
    }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.goal.stop(mob, cx);
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.goal.tick(mob, cx);
    }
}

fn unrolled(mob: &Mob) -> bool {
    phase(mob) == Phase::Unrolled
}

#[derive(Clone, Debug, Default)]
struct Flee {
    from: Vec3,
}

impl Goal for Flee {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if !scurry(mob, cx.w) {
            return false;
        }
        let player =
            cx.w.player
                .as_ref()
                .filter(|_| mob.can_target_player(cx.w, 6.0, true));
        let other =
            cx.w.nearby(mob.position, 6.0, |other| {
                other.alive && other.category != "misc" && !other.effects.contains(&MOLE)
            })
            .find(|other| {
                mob.distance_sqr(other.position) <= 36.0 && mob.sees(cx.w, Who::Mob(other.id))
            });
        let Some(from) = player
            .map(|player| player.position)
            .into_iter()
            .chain(other.map(|other| other.position))
            .min_by(|a, b| mob.distance_sqr(*a).total_cmp(&mob.distance_sqr(*b)))
        else {
            return false;
        };
        let Some(pos) = mob.default_random_pos_away(cx.w, 16, 7, from.components()) else {
            return false;
        };
        let away = Vec3::new(pos[0], pos[1], pos[2]).subtract(from);
        if away.length_squared() < mob.distance_sqr(from) {
            return false;
        }
        mob.ai.pending_path = mob.create_path_to(cx.w, pos.map(|coord| coord.floor() as i32), 0);
        self.from = from;
        mob.ai.pending_path.is_some()
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        scurry(mob, cx.w) && !mob.ai.nav.is_done()
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let path = mob.ai.pending_path.take();
        mob.nav_move_to_path(cx.w, path, 1.0);
    }
    fn tick(&mut self, mob: &mut Mob, _: &mut Cx) {
        mob.ai.nav.speed_modifier = if mob.distance_sqr(self.from) < 49.0 {
            1.5
        } else {
            1.0
        };
    }
}

pub fn register(mob: &mut Mob) {
    mob.ai.goals.clear();
    if mob.kind.id == TRAIL {
        return;
    }
    let food: Arc<[String]> = [FOOD.to_owned()].into();
    let kind = Arc::make_mut(&mut mob.kind);
    kind.food = food.clone();
    kind.tempt = food.clone();
    mob.ai.goals.add(0, Float);
    mob.ai.goals.add(1, Hide);
    mob.ai.goals.add(
        1,
        PhaseGoal {
            goal: Panic::new(2.0),
            allows: |mob| mob.fire_ticks > 0 || mob.ticks_frozen > 0,
            continue_checked: false,
        },
    );
    mob.ai.goals.add(2, Flee::default());
    mob.ai.goals.add(
        3,
        PhaseGoal {
            goal: Stroll::new(1.5).interval(1).position(scurry_position),
            allows: |mob| phase(mob) == Phase::Underground && state(&mob.ai.nav).hold_ticks == 0,
            continue_checked: true,
        },
    );
    mob.ai.goals.add(
        3,
        PhaseGoal {
            goal: Breed::new(1.0),
            allows: unrolled,
            continue_checked: true,
        },
    );
    mob.ai.goals.add(
        4,
        PhaseGoal {
            goal: Tempt::new(1.25, food, false),
            allows: unrolled,
            continue_checked: false,
        },
    );
    mob.ai.goals.add(
        5,
        PhaseGoal {
            goal: FollowParent::new(1.25),
            allows: unrolled,
            continue_checked: false,
        },
    );
    mob.ai.goals.add(6, Stroll::avoiding_water(1.0));
    mob.ai.goals.add(
        7,
        PhaseGoal {
            goal: LookAtPlayer::new(6.0),
            allows: unrolled,
            continue_checked: false,
        },
    );
    mob.ai.goals.add(
        8,
        PhaseGoal {
            goal: RandomLookAround::default(),
            allows: unrolled,
            continue_checked: false,
        },
    );
}

#[derive(Clone, Copy, Debug)]
pub struct WolfDigTick {
    pub mole: u32,
    pub elapsed: u8,
    pub ground: BlockStateId,
    pub hit_pitch: Option<f32>,
}

#[derive(Clone, Debug, Default)]
pub struct WolfDig {
    target: Option<u32>,
    elapsed: u8,
    ground: Option<BlockStateId>,
}

fn check_wolf_target<'a>(
    wolf: &Mob,
    world: &'a MobWorld<'_>,
    range_sqr: f64,
) -> Option<&'a super::Seen> {
    let Some(Who::Mob(id)) = wolf.ai.target else {
        return None;
    };
    world.seen(id).filter(|target| {
        target.alive
            && target.effects.contains(&DIGGABLE)
            && wolf.distance_sqr(target.position) <= range_sqr
    })
}

impl Goal for WolfDig {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE | LOOK | JUMP
    }
    fn every_tick(&self) -> bool {
        true
    }
    fn can_use(&mut self, wolf: &mut Mob, cx: &mut Cx) -> bool {
        wolf.on_ground && check_wolf_target(wolf, cx.w, 4.0).is_some()
    }
    fn can_continue(&mut self, wolf: &mut Mob, cx: &mut Cx) -> bool {
        self.elapsed < 50
            && check_wolf_target(wolf, cx.w, 6.25).map(|target| target.id) == self.target
            && self.target.is_some()
    }
    fn start(&mut self, wolf: &mut Mob, cx: &mut Cx) {
        let target = check_wolf_target(wolf, cx.w, 4.0);
        self.target = target.map(|mole| mole.id);
        self.ground = target.and_then(|mole| {
            let [x, y, z] = mole.position.components().map(|value| value.floor() as i32);
            let block = cx.w.world.block(x, y - 1, z);
            if cx.w.facts.is_air(block) || cx.w.block_collision_boxes(x, y - 1, z).is_empty() {
                cx.w.registry.default_state("minecraft:dirt")
            } else {
                Some(block)
            }
        });
        self.elapsed = 0;
        state_mut(&mut wolf.ai.nav).wolf_digging = true;
        cx.events.push(MobEvent::EntityEvent {
            mob: wolf.id,
            event: 101,
            at: wolf.position.components(),
        });
    }
    fn stop(&mut self, wolf: &mut Mob, cx: &mut Cx) {
        self.target = None;
        self.elapsed = 0;
        wolf.nav_stop();
        state_mut(&mut wolf.ai.nav).wolf_digging = false;
        cx.events.push(MobEvent::EntityEvent {
            mob: wolf.id,
            event: 102,
            at: wolf.position.components(),
        });
    }
    fn tick(&mut self, wolf: &mut Mob, cx: &mut Cx) {
        let Some(mole) = check_wolf_target(wolf, cx.w, 6.25) else {
            return;
        };
        freeze(wolf);
        wolf.look_at(mole.position.components());
        if let Some(ground) = self.ground {
            let hit_pitch = self.elapsed.is_multiple_of(4)
                .then(|| 0.7 + wolf.random.next_float() * 0.2);
            state_mut(&mut wolf.ai.nav).wolf_dig = Some(WolfDigTick {
                mole: mole.id, elapsed: self.elapsed, ground, hit_pitch,
            });
        }
        self.elapsed += 1;
    }
}
