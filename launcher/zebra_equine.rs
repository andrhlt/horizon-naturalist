use super::{Mob, MobEvent, equine};
use super::finalize::{Attr, set_base};
use super::goals::{Avoid, AvoidEntity, Breed, Cx, Float, FollowParent, Goal, LookAtPlayer,
    LOOK, MOVE, Panic, RandomLookAround, Stroll, Tempt, goal_clone};
use horizon_core::geometry::trig::Mth;
use horizon_core::random::RandomSource;
use std::sync::Arc;

pub const KIND: &str = "naturalist:zebra";

const LURES: [&str; 7] = ["minecraft:wheat", "minecraft:sugar", "minecraft:hay_block",
    "minecraft:apple", "minecraft:golden_carrot", "minecraft:golden_apple", "minecraft:enchanted_golden_apple"];

pub fn register(m: &mut Mob) {
    let tempt: Arc<[String]> = LURES.map(str::to_owned).into();
    let food: Arc<[String]> = LURES.into_iter().chain(["minecraft:carrot"]).map(str::to_owned).collect();
    let kind = Arc::make_mut(&mut m.kind);
    kind.tempt = tempt.clone();
    kind.food = food;
    kind.water_slowdown = 0.96;
    m.extra.vector[0] = m.attrs.jump_strength;
    let goals = &mut m.ai.goals;
    goals.add(0, Float);
    goals.add(1, Panic::new(1.25));
    goals.add(1, RunAround { target: [0.0; 3], ticks: 0 });
    goals.add(2, Breed::new(1.0));
    goals.add(3, ZebraTempt(Tempt::new(1.25, tempt, true)));
    goals.add(4, Flee(AvoidEntity::new(Avoid::Player, 16.0, 1.6, 1.6)));
    goals.add(5, FollowParent::new(1.0));
    goals.add(6, Stroll::avoiding_water(0.7));
    goals.add(7, LookAtPlayer::new(6.0));
    goals.add(8, RandomLookAround::default());
}

pub fn roll(r: &mut impl RandomSource) -> [f64; 3] {
    let health = 15 + r.next_int_bound(8).unwrap_or(0) + r.next_int_bound(9).unwrap_or(0);
    let speed = (0.5 + r.next_double() * 0.3 + r.next_double() * 0.3 + r.next_double() * 0.3) * 0.25;
    let jump = f64::from(0.4_f32) + r.next_double() * 0.1;
    [f64::from(health), speed, jump]
}

pub fn apply_roll(m: &mut Mob, [health, speed, jump]: [f64; 3]) {
    set_base(m, Attr::MaxHealth, health);
    set_base(m, Attr::MovementSpeed, speed);
    set_base(m, Attr::JumpStrength, jump);
    m.extra.vector[0] = jump;
}

pub fn finish_generated(m: &mut Mob, random: &mut impl RandomSource, group: &mut Option<super::finalize::Group>) {
    use super::finalize::{Ageable, Group, baby_start_age, mob_spawn, set_age};
    apply_roll(m, roll(random));
    if let Group::Ageable(ageable) = group.get_or_insert_with(|| Group::Ageable(Ageable::chance(0.2))) {
        if ageable.should_spawn_baby && ageable.size > 0 && random.next_float() <= ageable.baby_chance {
            let age = baby_start_age(m.species());
            set_age(m, age);
        }
        ageable.size += 1;
    }
    mob_spawn(m, random);
}

pub fn tame_tick(m: &mut Mob, events: &mut Vec<MobEvent>) {
    if m.extra.tame || m.rider.is_none() || m.random.next_int_bound(50).unwrap_or(1) != 0 {
        return;
    }
    if m.random.next_int_bound(equine::max_temper(m.species())).unwrap_or(0) < equine::temper(m) {
        m.extra.tame = true;
        events.push(MobEvent::Tamed { mob: m.id });
    } else {
        equine::modify_temper(m, 5);
        events.push(MobEvent::Buck { mob: m.id });
        equine::make_mad(m, events);
    }
}

#[derive(Clone, Debug)]
struct Flee(AvoidEntity);
impl Flee {
    fn allowed(m: &Mob) -> bool { !m.extra.tame && !equine::is_vehicle(m) }
}
impl Goal for Flee {
    goal_clone!();
    fn flags(&self) -> u8 { MOVE }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool { Self::allowed(m) && self.0.can_use(m, cx) }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool { Self::allowed(m) && self.0.can_continue(m, cx) }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) { self.0.start(m, cx); }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) { self.0.tick(m, cx); }
}

#[derive(Clone, Debug)]
struct ZebraTempt(Tempt);
impl Goal for ZebraTempt {
    goal_clone!();
    fn flags(&self) -> u8 { MOVE | LOOK }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        !equine::is_vehicle(m) && self.0.can_use(m, cx)
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        self.0.can_scare = !m.extra.tame;
        !equine::is_vehicle(m) && self.0.can_continue(m, cx)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) { self.0.start(m, cx); }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) { self.0.tick(m, cx); }
    fn stop(&mut self, m: &mut Mob, cx: &mut Cx) { self.0.stop(m, cx); }
}

#[derive(Clone, Debug)]
struct RunAround { target: [f64; 3], ticks: i32 }
impl Goal for RunAround {
    goal_clone!();
    fn flags(&self) -> u8 { MOVE }
    fn every_tick(&self) -> bool { true }
    fn can_use(&mut self, m: &mut Mob, _: &mut Cx) -> bool { !m.extra.tame && m.rider.is_some() }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        self.can_use(m, cx) && self.ticks > 0 &&
            (m.position.x - self.target[0]).powi(2) + (m.position.z - self.target[2]).powi(2) > 1.0
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.target = m.default_random_pos(cx.w, 5, 4).unwrap_or_else(|| {
            let angle = m.random.next_float() * std::f32::consts::TAU;
            let distance = 3.0 + m.random.next_double() * 2.0;
            [m.position.x + f64::from(angle.mcos()) * distance,
                m.position.y, m.position.z + f64::from(angle.msin()) * distance]
        });
        self.ticks = 60;
        m.nav_move_to(cx.w, self.target, 1.6);
    }
    fn tick(&mut self, m: &mut Mob, _: &mut Cx) {
        self.ticks -= 1;
        if m.ai.nav.is_done() {
            let [x, y, z] = self.target;
            m.ai.mov.set_wanted(x, y, z, 1.6);
        }
    }
    fn stop(&mut self, m: &mut Mob, _: &mut Cx) { m.nav_stop(); }
}
