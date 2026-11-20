use super::super::{
    Damage, DamageKind, Who,
    nav::naturalist_hippo_state::{self as hippo, KIND},
    path::{Path, PathType},
    tame::naturalist_duck_pet_goals,
};
use super::{
    Breed, Cx, FollowParent, Goal, HurtByTarget, JUMP, LOOK, LookAtPlayer, MOVE, MeleeAttack, Mob,
    MobEvent, NearestTarget, Panic, RandomLookAround, Stroll, TARGET, Tempt, goal_clone,
};
use horizon_core::{geometry::Vec3, random::RandomSource};

pub(in crate::mobs) fn register(m: &mut Mob) {
    let food: std::sync::Arc<[String]> = [String::from("minecraft:melon")].into();
    let kind = std::sync::Arc::make_mut(&mut m.kind);
    kind.food = food.clone();
    kind.tempt = food.clone();
    kind.water_slowdown = 0.98;
    m.set_malus(PathType::Water, 0.0);
    m.ai.nav.can_float = true;
    m.ai.goals.clear();
    m.ai.targets.clear();
    m.ai.goals.add(0, SmoothFloat);
    naturalist_duck_pet_goals::add_hippo_goals(m);
    m.ai.goals.add(1, WetBreed(Breed::new(1.0)));
    m.ai.goals.add(2, Tempt::new(1.0, food, false));
    m.ai.goals.add(3, AttackBoats::default());
    m.ai.goals.add(4, MeleeAttack::new(1.25, true));
    m.ai.goals.add(5, BabyPanic(Panic::new(1.25)));
    m.ai.goals.add(6, FollowHippo(FollowParent::new(1.25)));
    m.ai.goals.add(7, Stroll::swimming(1.0, 10));
    m.ai.goals.add(8, LookAtPlayer::new(6.0));
    m.ai.goals.add(9, RandomLookAround::default());
    m.ai.targets.add(
        1,
        HurtByTarget::new()
            .polar_bear()
            .alert_if(|seen| !seen.baby && seen.effects.contains(&KIND)),
    );
    m.ai.targets.add(
        4,
        NearestTarget::player(true).when(|m, world| {
            !m.is_baby()
                && world.player.as_ref().is_some_and(|player| {
                    m.extra.owner != Some(player.uuid)
                        && world.fluids(player.bounds, player.eye[1]).in_water
                })
        }),
    );
}

#[derive(Clone, Debug)]
struct SmoothFloat;
impl Goal for SmoothFloat {
    goal_clone!();
    fn flags(&self) -> u8 {
        JUMP
    }
    fn every_tick(&self) -> bool {
        true
    }
    fn can_use(&mut self, m: &mut Mob, _: &mut Cx) -> bool {
        m.fluids.in_water && m.fluids.water > m.fluid_jump_threshold() || m.fluids.lava > 0.0
    }
    fn start(&mut self, m: &mut Mob, _: &mut Cx) {
        m.no_gravity = true;
    }
    fn stop(&mut self, m: &mut Mob, _: &mut Cx) {
        m.no_gravity = false;
    }
    fn tick(&mut self, m: &mut Mob, _: &mut Cx) {
        if m.fluids.in_water
            && m.fluids.water > m.fluid_jump_threshold() + 0.1
            && m.velocity.y < 0.05
        {
            m.velocity.y = (m.velocity.y + 0.02).min(0.05);
        }
    }
}

#[derive(Clone, Debug)]
struct WetBreed(Breed);
impl Goal for WetBreed {
    goal_clone!();
    fn flags(&self) -> u8 {
        self.0.flags()
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if m.extra.in_love <= 0 || !m.fluids.in_water {
            return false;
        }
        let area = m.bounds().inflate(Vec3::new(8.0, 8.0, 8.0));
        self.0.partner =
            cx.w.others_near(area)
                .filter(|seen| {
                    seen.id != m.id
                        && seen.alive
                        && seen.in_love
                        && seen.in_water
                        && !seen.panicking
                        && seen.effects.contains(&KIND)
                        && seen.bounds.intersects(area)
                })
                .min_by(|a, b| {
                    m.distance_sqr(a.position)
                        .total_cmp(&m.distance_sqr(b.position))
                })
                .map(|seen| seen.id);
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
struct FollowHippo(FollowParent);
impl Goal for FollowHippo {
    goal_clone!();
    fn flags(&self) -> u8 {
        self.0.flags()
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if !m.is_baby() || m.extra.tame {
            return false;
        }
        let area = m.bounds().inflate(Vec3::new(8.0, 2.0, 8.0));
        let mut best: Option<(u32, f64)> = None;
        for candidate in cx.w.others_near(area).filter(|s| {
            s.alive && !s.baby && s.effects.contains(&KIND) && s.bounds.intersects(area)
        }) {
            let d = m.distance_sqr(candidate.position);
            if best.is_none_or(|(_, previous)| d <= previous) {
                best = Some((candidate.id, d));
            }
        }
        self.0.parent = best
            .filter(|(_, distance)| *distance >= 25.0)
            .map(|(id, _)| id);
        self.0.parent.is_some()
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        !m.extra.tame
            && self.0.can_continue(m, cx)
            && self
                .0
                .parent
                .and_then(|id| cx.w.seen(id))
                .is_some_and(|s| m.distance_sqr(s.position) >= 25.0)
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

#[derive(Clone, Debug, Default)]
struct AttackBoats {
    target: Option<u32>,
    selected_path: Option<Path>,
    last_check: i64,
    pathed_to: Vec3,
    path_cooldown: i32,
    attack_cooldown: i32,
}

fn boat_reach(mob: &Mob, boat: hippo::BoatTarget) -> f64 {
    let body = mob.width() * 1.2;
    f64::from(body * body + boat.bounds.size().x as f32)
}

impl AttackBoats {
    fn current(&self, mob: &Mob) -> Option<hippo::BoatTarget> {
        let id = self.target?;
        hippo::state(&mob.ai.nav)
            .boats
            .iter()
            .copied()
            .find(|boat| boat.id == id)
    }
}

impl Goal for AttackBoats {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE | LOOK | TARGET
    }
    fn every_tick(&self) -> bool {
        true
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if m.is_baby() || cx.w.game_time - self.last_check < 20 {
            return false;
        }
        self.last_check = cx.w.game_time;
        let area = m.bounds().inflate(Vec3::new(8.0, 4.0, 8.0));
        let nearby = hippo::state(&m.ai.nav)
            .boats
            .iter()
            .find(|boat| boat.bounds.intersects(area));
        if let Some(boat) = nearby {
            self.target = Some(boat.id);
        }
        let Some(target) = self.current(m) else {
            return false;
        };
        let block = target.position.components().map(|v| v.floor() as i32);
        self.selected_path = m.create_path_to(cx.w, block, 0);
        self.selected_path.is_some() || m.distance_sqr(target.position) <= boat_reach(m, target)
    }
    fn can_continue(&mut self, m: &mut Mob, _: &mut Cx) -> bool {
        self.current(m).is_some() && !m.ai.nav.is_done()
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        m.nav_move_to_path(cx.w, self.selected_path.clone(), 1.25);
        m.ai.aggressive = true;
        self.path_cooldown = 0;
        self.attack_cooldown = 0;
    }
    fn stop(&mut self, m: &mut Mob, cx: &mut Cx) {
        if m.target_info(cx.w)
            .is_none_or(|target| !target.alive || target.creative)
        {
            m.ai.target = None;
        }
        m.ai.aggressive = false;
        m.nav_stop();
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let Some(boat) = self.current(m) else { return };
        let p = boat.position;
        let eye_y = p.y + boat.bounds.size().y * 0.85;
        m.look_at_speed([p.x, eye_y, p.z], 30.0, 30.0);
        let distance = m.distance_sqr(p);
        self.path_cooldown = (self.path_cooldown - 1).max(0);
        let eye = Vec3::new(m.position.x, m.eye_y(), m.position.z);
        if self.path_cooldown == 0
            && cx.w.line_of_sight(eye, Vec3::new(p.x, eye_y, p.z))
            && (self.pathed_to == Vec3::ZERO
                || p.subtract(self.pathed_to).length_squared() >= 1.0
                || m.random.next_float() < 0.05)
        {
            self.pathed_to = p;
            self.path_cooldown = 4 + m.random.next_int_bound(7).unwrap_or(0);
            if distance > 1024.0 {
                self.path_cooldown += 10;
            } else if distance > 256.0 {
                self.path_cooldown += 5;
            }
            if !m.nav_move_to_entity(cx.w, p.components(), 0, 1.25) {
                self.path_cooldown += 15;
            }
        }
        self.attack_cooldown = (self.attack_cooldown - 1).max(0);
        if self.attack_cooldown == 0 && distance <= boat_reach(m, boat) {
            self.attack_cooldown = 20;
            m.swing();
            cx.events.push(MobEvent::AttackMob {
                target: boat.id,
                amount: super::super::gear::attack_damage(m, cx.w.items),
                source: Damage {
                    kind: DamageKind::MobAttack,
                    attacker: Some(Who::Mob(m.id)),
                    from: Some(m.position.components()),
                    attacker_species: Some(m.species()),
                },
            });
        }
    }
}
