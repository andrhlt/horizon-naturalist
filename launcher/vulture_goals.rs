use super::{
    Cx, Float, Goal, JUMP, LOOK, LookAtPlayer, MOVE, MeleeAttack, Mob, MobWorld, MoveToBlock,
    NearestTarget, RandomLookAround, Species, Who, goal_clone, reduced,
};
use horizon_core::{geometry::Vec3, random::RandomSource};
use std::sync::Arc;

pub const KIND: &str = "naturalist:vulture";
pub const FOOD: &str = "minecraft:rotten_flesh";

#[derive(Clone, Debug, Default)]
pub struct VultureState {
    pub food_pos: Option<[f64; 3]>,
    pub vulnerable_players: Arc<Vec<[i32; 4]>>,
    pub steal: Option<[i32; 4]>,
    pub perch_target: Option<[i32; 3]>,
    pub perch_reservations: Arc<Vec<(u32, [i32; 3])>>,
    pub perched_ticks: i32,
    pub since_eating: i32,
    perch_cooldown: i32,
}

pub fn state(m: &Mob) -> &VultureState {
    m.ai.nav.__horizon_mixins_naturalist_vulture_state()
}

pub fn state_mut(m: &mut Mob) -> &mut VultureState {
    m.ai.nav.__horizon_mixins_naturalist_vulture_state_mut()
}

fn has_flesh(m: &Mob, _: &MobWorld) -> bool {
    m.extra.main_hand.as_deref() == Some(FOOD)
}

fn can_steal(m: &Mob, w: &MobWorld) -> bool {
    w.player
        .as_ref()
        .is_some_and(|p| p.main_hand.is_some() && state(m).vulnerable_players.contains(&p.uuid))
}

pub fn register(m: &mut Mob) {
    use super::super::path::PathType;
    super::naturalist_insect_goals::configure_small_flyer(m);
    m.extra.pick_up_loot = true;
    m.ai.nav.can_float = true;
    for (kind, malus) in [
        (PathType::FireInNeighbor, -1.0),
        (PathType::Cocoa, -1.0),
        (PathType::DamagingInNeighbor, 0.0),
        (PathType::Damaging, 0.0),
    ] {
        m.set_malus(kind, malus);
    }
    let mut cactus = MoveToBlock::new(1.0, 32, 32);
    cactus.next_start = m.random.next_int_bound(1200).unwrap_or(0);
    m.ai.goals.add(0, Float);
    m.ai.goals.add(1, FleePlayer::default());
    m.ai.goals.add(1, Attack(MeleeAttack::new(1.2, true)));
    m.ai.goals.add(2, Scavenge);
    m.ai.goals.add(2, Perch(cactus));
    m.ai.goals.add(3, HighFlight::default());
    m.ai.goals.add(4, LookAtPlayer::new(8.0));
    m.ai.goals.add(5, RandomLookAround::default());
    m.ai.targets.clear();
    m.ai.targets.add(
        1,
        NearestTarget::species(
            &[
                Species::Zombie,
                Species::ZombieVillager,
                Species::Husk,
                Species::ZombifiedPiglin,
                Species::Zoglin,
            ],
            false,
        )
        .when(|m, w| !has_flesh(m, w)),
    );
    m.ai.targets.add(
        2,
        NearestTarget::player(false).when(|m, w| !has_flesh(m, w) && can_steal(m, w)),
    );
}

pub fn startle(m: &mut Mob, from: Option<Vec3>) {
    let flight = state_mut(m);
    flight.perched_ticks = 0;
    flight.perch_cooldown = 200;
    let away = from.map_or(Vec3::ZERO, |at| {
        Vec3::new(m.position.x - at.x, 0.0, m.position.z - at.z)
    });
    let away = if away.length_squared() > 1.0e-4 {
        away.normalize().scale(0.4)
    } else {
        Vec3::ZERO
    };
    m.velocity = Vec3::new(away.x, 0.5, away.z);
    m.needs_sync = true;
}

pub fn tick(m: &mut Mob) {
    if !m.no_ai && m.alive() {
        let state = state_mut(m);
        state.perch_cooldown = (state.perch_cooldown - 1).max(0);
        state.since_eating += 1;
    }
}

#[derive(Clone, Debug)]
struct Attack(MeleeAttack);

impl Attack {
    fn permitted(&self, m: &Mob, w: &MobWorld) -> bool {
        m.extra.main_hand.is_none() && (m.ai.target != Some(Who::Player) || can_steal(m, w))
    }
}

impl Goal for Attack {
    goal_clone!();
    fn flags(&self) -> u8 {
        self.0.flags()
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        self.permitted(m, cx.w) && self.0.can_use(m, cx)
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        self.permitted(m, cx.w) && self.0.can_continue(m, cx)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.start(m, cx);
    }
    fn stop(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.stop(m, cx);
    }
    fn every_tick(&self) -> bool {
        true
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        if m.ai.target != Some(Who::Player) {
            self.0.tick(m, cx);
            return;
        }
        let Some(player) = cx.w.player.as_ref() else {
            return;
        };
        if m.target_info(cx.w)
            .is_some_and(|target| m.within_melee_range(cx.w, &target))
        {
            m.swing();
            state_mut(m).steal = Some(player.uuid);
            m.ai.target = None;
            m.ai.aggressive = false;
        } else {
            let at = player.position;
            m.look_at(player.eye);
            m.nav_move_to_entity(cx.w, [at.x, at.y, at.z], 0, 1.2);
        }
    }
}

#[derive(Clone, Debug, Default)]
struct FleePlayer {
    player: Option<[i32; 4]>,
    recalc: i32,
}

impl FleePlayer {
    fn flee(&self, m: &mut Mob, cx: &mut Cx) {
        let Some(player) = cx.w.player.as_ref() else {
            return;
        };
        let at = player.position;
        if let Some(target) = m.default_random_pos_away(cx.w, 16, 7, [at.x, at.y, at.z])
            && m.distance_sqr(at)
                <= ([target[0] - at.x, target[1] - at.y, target[2] - at.z])
                    .into_iter()
                    .map(|d| d * d)
                    .sum::<f64>()
            && m.nav_move_to(cx.w, target, 1.4)
        {
            return;
        }
        let away = Vec3::new(m.position.x - at.x, 0.0, m.position.z - at.z);
        let [x, _, z] = if away.length_squared() < 1.0e-4 {
            m.view_vector()
        } else {
            let unit = away.normalize();
            [unit.x, 0.0, unit.z]
        };
        m.ai.mov.set_wanted(
            m.position.x + x * 12.0,
            m.position.y + 8.0,
            m.position.z + z * 12.0,
            1.4,
        );
    }
}

impl Goal for FleePlayer {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        let Some(player) = cx.w.player.as_ref() else {
            return false;
        };
        if m.ai.target.is_some()
            || !player.alive
            || player.creative
            || !m.player_in_range(cx.w, 16.0, true, true)
        {
            return false;
        }
        self.player = Some(player.uuid);
        true
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        cx.w.player.as_ref().is_some_and(|p| {
            Some(p.uuid) == self.player && p.alive && m.distance_sqr(p.position) < 256.0
        })
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        startle(m, cx.w.player.as_ref().map(|p| p.position));
        self.recalc = 20;
        self.flee(m, cx);
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.recalc -= 1;
        if self.recalc <= 0 {
            self.recalc = 20;
            self.flee(m, cx);
        }
    }
    fn stop(&mut self, _: &mut Mob, _: &mut Cx) {
        self.player = None;
    }
}

#[derive(Clone, Debug)]
struct Scavenge;

impl Goal for Scavenge {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        !has_flesh(m, cx.w) && state(m).food_pos.is_some()
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.tick(m, cx);
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        if let Some(at) = state(m).food_pos {
            m.nav_move_to_entity(cx.w, at, 0, 1.2);
        }
    }
}

#[derive(Clone, Debug)]
struct Perch(MoveToBlock);

fn valid_cactus(w: &MobWorld, [x, y, z]: [i32; 3]) -> bool {
    w.block_name(x, y, z) == "minecraft:cactus" && w.facts.is_air(w.world.block(x, y + 1, z))
}

impl Goal for Perch {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE | JUMP
    }
    fn set_start_delay(&mut self, ticks: i32) {
        self.0.next_start = ticks;
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if m.ai.target.is_some() || has_flesh(m, cx.w) || state(m).perch_cooldown > 0 {
            return false;
        }
        if self.0.next_start > 0 {
            self.0.next_start -= 1;
            return false;
        }
        self.0.next_start = reduced(200 + m.random.next_int_bound(200).unwrap_or(0));
        self.0
            .may_find(m, cx.w, |b| cx.w.facts.get(b).name.as_ref() == "minecraft:cactus")
            && self.0.find_nearest(m, cx.w, |w, pos| {
                valid_cactus(w, pos)
                    && !state(m)
                        .perch_reservations
                        .iter()
                        .any(|&(id, used)| id != m.id && used == pos)
            })
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        state(m).perched_ticks < 300
            && m.ai.target.is_none()
            && state(m).perch_cooldown == 0
            && self.0.can_continue(cx.w, valid_cactus)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.start(m, cx.w);
        state_mut(m).perch_target = Some(self.0.block);
    }
    fn stop(&mut self, m: &mut Mob, _: &mut Cx) {
        let flight = state_mut(m);
        flight.perch_target = None;
        flight.perched_ticks = 0;
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.tick(m, cx.w);
        if self.0.reached {
            let [x, y, z] = self.0.block;
            let at = Vec3::new(f64::from(x) + 0.5, f64::from(y + 1), f64::from(z) + 0.5);
            let offset = at.subtract(m.position);
            m.nav_stop();
            if offset.length_squared() > 0.04 {
                m.velocity = offset.scale(0.25);
            } else {
                m.position = at;
                m.velocity = Vec3::ZERO;
            }
            state_mut(m).perched_ticks += 1;
        }
    }
}

#[derive(Clone, Debug, Default)]
struct HighFlight {
    altitude: i32,
    attempts: i32,
}

impl Goal for HighFlight {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE
    }
    fn can_use(&mut self, m: &mut Mob, _: &mut Cx) -> bool {
        m.ai.nav.is_done() && m.random.next_int_bound(10) == Ok(0)
    }
    fn can_continue(&mut self, m: &mut Mob, _: &mut Cx) -> bool {
        !m.ai.nav.is_done()
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        let [x, _, z] = m.view_vector();
        if self.altitude < 0 || self.attempts <= 0 {
            let w = cx.w;
            let ground = (w.world.bottom_y()..w.world.top_y())
                .rev()
                .find(|&y| {
                    !w.facts.is_air(w.world.block(
                        m.position.x.floor() as i32,
                        y,
                        m.position.z.floor() as i32,
                    ))
                })
                .map_or(w.world.bottom_y(), |y| y + 1);
            let low = ground + 8;
            let high = (ground + 20).min(w.world.top_y() - 33);
            self.altitude = low + m.random.next_int_bound(high - low + 1).unwrap_or(0);
            self.attempts = 16 + m.random.next_int_bound(16).unwrap_or(0);
        } else {
            self.attempts -= 1;
        }
        let pos = [
            m.position.x + x * 8.0 + f64::from(m.random.next_int_bound(5).unwrap_or(0) - 2),
            f64::from(self.altitude),
            m.position.z + z * 8.0 + f64::from(m.random.next_int_bound(5).unwrap_or(0) - 2),
        ];
        m.nav_move_to_entity(cx.w, pos.map(f64::floor), 1, 1.0);
    }
}
