use super::{
    Breed, Cx, Float, FollowParent, Goal, JUMP, LOOK, LookAtPlayer, MOVE, Mob, MobWorld, Panic,
    RandomLookAround, Stroll, Tempt, goal_clone, reduced,
};
use crate::{
    items::ItemStack,
    mobs::{nav::NavKind, tame::naturalist_duck_pet_goals},
};
use horizon_core::{geometry::Vec3, random::RandomSource};
use std::sync::Arc;

pub const KIND: &str = "naturalist:rat";
pub const FOOD: &str = "minecraft:bread";
pub const WORK_RADIUS: i32 = 7;
pub const SLOTS: usize = 27;
type Position = [i32; 3];

#[derive(Clone, Copy, Debug)]
pub enum WorkAction {
    Harvest(Position),
    Deposit(Position, i32),
    Abandon(Position),
}

#[derive(Clone, Debug, Default)]
pub struct State {
    pub station: Option<Position>,
    pub cargo: Vec<Option<ItemStack>>,
    pub station_exists: Option<bool>,
    pub action: Option<WorkAction>,
    pub deposited: Option<bool>,
    pub sleeping: bool,
    pub interested: bool,
    pub from_hand: bool,
}

impl State {
    pub fn has_cargo(&self) -> bool {
        self.cargo.iter().any(Option::is_some)
    }

    fn carry_full(&self) -> bool {
        self.cargo.len() == SLOTS && self.cargo.iter().all(Option::is_some)
    }
}

pub fn state(mob: &Mob) -> &State {
    mob.ai.nav.__horizon_mixins_naturalist_rat_state()
}

pub fn state_mut(mob: &mut Mob) -> &mut State {
    mob.ai.nav.__horizon_mixins_naturalist_rat_state_mut()
}

pub fn set_station(mob: &mut Mob, station: Option<Position>) {
    state_mut(mob).station = station;
    mob.ai.home = station.map(|position| (position, WORK_RADIUS));
}

pub fn render_pose(mob: &Mob) -> u8 {
    let status = state(mob);
    u8::from(mob.extra.sitting_pose)
        | (u8::from(status.sleeping) << 1)
        | (u8::from(mob.fluids.in_water) << 2)
        | (u8::from(status.interested) << 3)
        | (u8::from(mob.extra.sprinting) << 4)
}

pub fn register(mob: &mut Mob) {
    let food: Arc<[String]> = [FOOD.to_owned()].into();
    let kind = Arc::make_mut(&mut mob.kind);
    kind.food = food.clone();
    kind.tempt = food.clone();
    kind.flags.climber = true;
    mob.ai.nav.kind = NavKind::WallClimber;
    mob.ai.goals.clear();
    mob.ai.targets.clear();
    mob.ai.goals.add(0, Float);
    naturalist_duck_pet_goals::add_rat_goals(mob);
    mob.ai.goals.add(1, Panic::new(2.0));
    mob.ai.goals.add(2, Breed::new(1.0));
    mob.ai.goals.add(3, Sleep);
    mob.ai.goals.add(4, Beg::default());
    mob.ai.goals.add(5, Tempt::new(1.25, food, false));
    mob.ai.goals.add(6, HarvestCrops::default());
    mob.ai.goals.add(6, FollowParent::new(1.1));
    mob.ai.goals.add(8, Stroll::avoiding_water(1.0));
    mob.ai
        .goals
        .add(9, LookAtPlayer::new(6.0).probability(0.02));
    mob.ai.goals.add(10, RandomLookAround::default());
}

pub fn can_sleep(mob: &Mob, world: &MobWorld<'_>) -> bool {
    !world.daylight
        && !world.thundering
        && !mob.fluids.in_water
        && !mob.is_on_fire()
        && mob.on_ground
        && !mob.extra.sitting
        && !mob.extra.passenger
        && mob.extra.mount.is_none()
        && !world
            .others_near(mob.bounds().inflate(Vec3::new(12.0, 6.0, 12.0)))
            .any(|seen| seen.id != mob.id && !seen.effects.contains(&KIND))
}

#[derive(Clone, Debug)]
struct Sleep;
impl Goal for Sleep {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE | LOOK | JUMP
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.ai.xxa == 0.0
            && mob.ai.yya == 0.0
            && mob.ai.zza == 0.0
            && (can_sleep(mob, cx.w) || state(mob).sleeping)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        can_sleep(mob, cx.w)
    }
    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        state_mut(mob).sleeping = true;
        mob.ai.jump = false;
        mob.ai.jumping = false;
        mob.nav_stop();
        mob.ai
            .mov
            .set_wanted(mob.position.x, mob.position.y, mob.position.z, 0.0);
    }
    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        state_mut(mob).sleeping = false;
    }
}

#[derive(Clone, Debug, Default)]
struct Beg {
    look_time: i32,
}
impl Beg {
    fn holding_food(player: &crate::mobs::PlayerView) -> bool {
        [player.main_hand.as_deref(), player.off_hand.as_deref()].contains(&Some(FOOD))
    }
}
impl Goal for Beg {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE | LOOK
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        cx.w.player.as_ref().is_some_and(|player| {
            let range = (4.0 * player.visibility_percent(Some(KIND), cx.w.items)).max(2.0);
            player.alive
                && !player.spectator
                && mob.distance_sqr(player.position) <= range * range
                && Self::holding_food(player)
                && cx.w.line_of_sight_for(
                    Some(mob.position.y),
                    Vec3::new(mob.position.x, mob.eye_y(), mob.position.z),
                    Vec3::new(player.eye[0], player.eye[1], player.eye[2]),
                )
        })
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        self.look_time > 0
            && cx.w.player.as_ref().is_some_and(|player| {
                player.alive
                    && mob.distance_sqr(player.position) <= 25.0
                    && Self::holding_food(player)
            })
    }
    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        state_mut(mob).interested = true;
        mob.nav_stop();
        self.look_time = reduced(40 + mob.random.next_int_bound(40).unwrap_or(0));
    }
    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        state_mut(mob).interested = false;
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        if let Some(player) = &cx.w.player {
            mob.look_at_speed(player.eye, 10.0, mob.max_head_x_rot());
            self.look_time -= 1;
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Phase {
    Harvest(Position),
    Return(Position),
    Deposit(Position),
}
impl Phase {
    fn position(self) -> Position {
        match self {
            Self::Harvest(pos) | Self::Return(pos) | Self::Deposit(pos) => pos,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct HarvestCrops {
    phase: Option<Phase>,
    pending: Option<Position>,
    cooldown: i32,
    path_ticks: i32,
    delay: i32,
    deposit_step: i32,
}

#[rustfmt::skip]
pub fn ripe(w: &MobWorld<'_>, [x, y, z]: Position) -> bool {
    let Some(state) = w.registry.state(w.world.block(x, y, z)) else { return false };
    let max = match state.name.as_str() {
        "minecraft:wheat" | "minecraft:carrots" | "minecraft:potatoes" => "7",
        "minecraft:beetroots" | "minecraft:nether_wart" => "3",
        _ => return false,
    };
    state.properties.get("age").is_some_and(|age| age == max)
}

fn nearest_crop(mob: &Mob, world: &MobWorld<'_>) -> Option<Position> {
    let [x, y, z] = state(mob).station?;
    let mut best = None;
    let mut nearest = f64::MAX;
    let block = mob.position.components().map(f64::floor);
    for dx in -WORK_RADIUS..=WORK_RADIUS {
        for dz in -WORK_RADIUS..=WORK_RADIUS {
            for dy in -2..=2 {
                let candidate = [x + dx, y + dy, z + dz];
                let distance = candidate
                    .iter()
                    .zip(block)
                    .map(|(&axis, rat)| (f64::from(axis) - rat).powi(2))
                    .sum();
                if distance < nearest && ripe(world, candidate) {
                    nearest = distance;
                    best = Some(candidate);
                }
            }
        }
    }
    best
}

fn can_work(mob: &Mob) -> bool {
    state(mob).station.is_some()
        && mob.extra.tame
        && !mob.is_baby()
        && !mob.extra.sitting
        && !state(mob).sleeping
        && !mob.extra.passenger
        && mob.extra.mount.is_none()
        && !mob.fluids.in_water
}

impl HarvestCrops {
    fn select(&mut self, mob: &mut Mob, cx: &mut Cx, crop: Option<Position>) {
        self.path_ticks = 0;
        let status = state(mob);
        self.phase = crop
            .filter(|_| !status.carry_full())
            .map(Phase::Harvest)
            .or_else(|| {
                status
                    .has_cargo()
                    .then_some(status.station)
                    .flatten()
                    .map(Phase::Return)
            });
        self.move_to(mob, cx);
    }
    fn move_to(&self, mob: &mut Mob, cx: &mut Cx) {
        if let Some(phase) = self.phase {
            let [x, y, z] = phase.position().map(f64::from);
            mob.nav_move_to(cx.w, [x + 0.5, y, z + 0.5], 1.2);
        }
    }
}

impl Goal for HarvestCrops {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE | LOOK
    }
    fn every_tick(&self) -> bool {
        true
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if !can_work(mob) {
            return false;
        }
        if self.cooldown > 0 {
            self.cooldown -= 1;
            return false;
        }
        self.cooldown = reduced(20 + mob.random.next_int_bound(20).unwrap_or(0));
        let data = state(mob);
        if data.station_exists == Some(false)
            && let Some(station) = data.station
        {
            state_mut(mob).action = Some(WorkAction::Abandon(station));
            return false;
        }
        self.pending = nearest_crop(mob, cx.w);
        self.pending.is_some() || state(mob).has_cargo()
    }
    fn can_continue(&mut self, mob: &mut Mob, _: &mut Cx) -> bool {
        self.phase.is_some() && can_work(mob)
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.delay = 0;
        let crop = self.pending.take();
        self.select(mob, cx, crop);
    }
    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        self.phase = None;
        self.delay = 0;
        self.cooldown = reduced(10);
        mob.nav_stop();
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(phase) = self.phase else { return };
        let pos = phase.position();
        let center = pos.map(|axis| f64::from(axis) + 0.5);
        mob.look_at(center);
        if let Some(success) = state_mut(mob).deposited.take() {
            if !success {
                self.phase = None;
                return;
            }
            self.deposit_step += 1;
            self.delay = 3;
        }
        if self.delay > 0 {
            self.delay -= 1;
            return;
        }
        if matches!(phase, Phase::Deposit(_)) {
            state_mut(mob).action = Some(WorkAction::Deposit(pos, self.deposit_step));
        } else if matches!(phase, Phase::Harvest(_)) && !ripe(cx.w, pos) {
            let crop = nearest_crop(mob, cx.w);
            self.select(mob, cx, crop);
        } else if mob.distance_sqr(Vec3::new(center[0], center[1], center[2])) > 4.0 {
            self.path_ticks += 1;
            if self.path_ticks > 200 {
                if matches!(phase, Phase::Harvest(_)) {
                    let crop = nearest_crop(mob, cx.w);
                    self.select(mob, cx, crop);
                } else {
                    self.phase = None;
                }
            } else if mob.ai.nav.is_done() {
                self.move_to(mob, cx);
            }
        } else {
            mob.nav_stop();
            match phase {
                Phase::Harvest(_) => {
                    state_mut(mob).action = Some(WorkAction::Harvest(pos));
                    self.delay = 10 + mob.random.next_int_bound(10).unwrap_or(0);
                }
                Phase::Return(_) => {
                    self.phase = Some(Phase::Deposit(pos));
                    self.deposit_step = 0;
                    state_mut(mob).action = Some(WorkAction::Deposit(pos, 0));
                }
                Phase::Deposit(_) => {}
            }
        }
    }
}
