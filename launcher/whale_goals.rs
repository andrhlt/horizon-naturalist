use super::super::{
    MobWorld,
    control::{LookKind, MoveKind},
    nav::{NavKind, naturalist_whale_state as whale},
    path::PathType,
};
use super::{Cx, FollowParent, Goal, MOVE, Mob, MobEvent, RandomLookAround, goal_clone};
use horizon_core::{
    geometry::{
        Vec3,
        trig::{DEG_TO_RAD, Mth},
    },
    random::RandomSource,
};

pub(in crate::mobs) fn register(mob: &mut Mob) {
    let kind = std::sync::Arc::make_mut(&mut mob.kind);
    kind.food = [
        "minecraft:cod",
        "minecraft:cooked_cod",
        "minecraft:salmon",
        "minecraft:cooked_salmon",
        "naturalist:crab_meat",
        "naturalist:cooked_crab_meat",
    ]
    .map(String::from)
    .into();
    kind.tempt = Default::default();
    kind.water_slowdown = 0.9;
    mob.set_malus(PathType::Water, 0.0);
    mob.set_malus(PathType::WaterBorder, 16.0);
    mob.ai.nav.kind = NavKind::Water { breach: false };
    mob.ai.mov.kind = MoveKind::Custom;
    mob.ai.look.kind = LookKind::SmoothSwimming(6.0);
    mob.ai.goals.clear();
    mob.ai.targets.clear();
    let surface_start = 200 + mob.random.next_int_bound(400).unwrap_or(0);
    let dive_start = 400 + mob.random.next_int_bound(400).unwrap_or(0);
    mob.ai.goals.add(0, FollowParent::new(1.1));
    mob.ai.goals.add(1, SeekDeep::default());
    mob.ai.goals.add(
        2,
        Surface {
            next: surface_start as u64,
            ..Default::default()
        },
    );
    mob.ai.goals.add(
        3,
        Dive {
            next: dive_start as u64,
            ..Default::default()
        },
    );
    mob.ai.goals.add(4, Swim::default());
    mob.ai.goals.add(5, RandomLookAround::default());
}

fn at_height(position: Vec3, y: f64) -> Vec3 {
    Vec3::new(position.x, y, position.z)
}

fn offset(position: Vec3, x: f64, y: f64, z: f64) -> Vec3 {
    position.add_vector(Vec3::new(x, y, z))
}

fn is_air(world: &MobWorld<'_>, position: Vec3) -> bool {
    let [x, y, z] = position
        .components()
        .map(|coordinate| coordinate.floor() as i32);
    world.facts.is_air(world.world.block(x, y, z))
}

fn want(mob: &mut Mob, target: Vec3) {
    mob.ai.mov.set_wanted(target.x, target.y, target.z, 1.2);
}

#[derive(Clone, Debug, Default)]
struct SeekDeep {
    target: Option<Vec3>,
    run: u16,
    retarget: u8,
}

impl Goal for SeekDeep {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE
    }
    fn every_tick(&self) -> bool {
        true
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.fluids.in_water
            && !whale::wet(cx.w, offset(mob.position, 0.0, 1.5, 0.0))
            && mob.velocity.length_squared() < 0.01
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        self.run < 400
            && mob.fluids.in_water
            && !whale::wet(cx.w, offset(mob.position, 0.0, 2.0, 0.0))
    }
    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        *self = Self::default();
        mob.nav_stop();
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.run += 1;
        self.retarget = self.retarget.saturating_sub(1);
        if self.retarget == 0 || self.target.is_none() {
            self.retarget = 40;
            let angle = mob.random.next_float() * std::f32::consts::TAU;
            let origin = mob.position;
            let sample = (4..=24)
                .step_by(4)
                .flat_map(|radius| {
                    (0..12).map(move |step| {
                        let heading = angle + step as f32 / 12.0 * std::f32::consts::TAU;
                        let radius = radius as f32;
                        offset(
                            origin,
                            f64::from(heading.msin() * radius),
                            0.0,
                            f64::from(heading.mcos() * radius),
                        )
                    })
                })
                .find(|position| {
                    whale::wet(cx.w, *position)
                        && whale::wet(cx.w, offset(*position, 0.0, -2.0, 0.0))
                });
            if let Some(position) = sample {
                self.target = Some(offset(position, 0.0, -2.0, 0.0));
            }
        }
        if let Some(target) = self.target {
            want(mob, target);
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Surface {
    next: u64,
    run: u16,
    skim: u8,
    lunge: u8,
    breach: bool,
    lunged: bool,
    surface: f64,
}

impl Surface {
    fn find_surface(&mut self, mob: &Mob, world: &MobWorld<'_>) -> bool {
        let top = (mob.position.y + f64::from(mob.height())).floor();
        for step in 0..48 {
            let point = at_height(mob.position, top + f64::from(step));
            if !whale::wet(world, point) {
                self.surface = point.y;
                return is_air(world, point);
            }
        }
        false
    }
}

impl Goal for Surface {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE
    }
    fn every_tick(&self) -> bool {
        true
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.fluids.in_water
            && !whale::motion(&mob.ai.nav).is_some_and(|state| state.diving)
            && u64::from(mob.age) >= self.next
            && self.find_surface(mob, cx.w)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if self.run >= 600 || self.skim >= 60 {
            return false;
        }
        if self.lunged {
            return self.lunge < 60
                && (self.lunge < 10 || !mob.fluids.in_water || mob.velocity.y > 0.0);
        }
        !whale::grinding(mob) && mob.fluids.in_water && self.find_surface(mob, cx.w)
    }
    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        self.run = 0;
        self.skim = 0;
        self.lunge = 0;
        self.lunged = false;
        self.breach = mob.random.next_float() < 0.3;
        mob.nav_stop();
    }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) {
        if self.lunged && mob.fluids.in_water {
            cx.events.push(MobEvent::Particles {
                kind: "naturalist:whale_splash",
                at: mob.center(),
                count: 80,
            });
            if !mob.silent {
                cx.events.push(MobEvent::Sound(
                    "entity.generic.splash",
                    mob.position.components(),
                    2.0,
                    0.6,
                ));
            }
        }
        let (base, span) = if self.skim > 0 || self.lunged {
            (600, 900)
        } else {
            (150, 150)
        };
        self.next = u64::from(mob.age) + base + mob.random.next_int_bound(span).unwrap_or(0) as u64;
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.run += 1;
        if self.lunged {
            self.lunge += 1;
            return;
        }
        let facing = whale::forward(mob.body_yaw);
        let head_y = mob.position.y + f64::from(mob.height());
        if self.breach && self.surface - head_y < 2.0 {
            if whale::wet(cx.w, mob.position.add_vector(facing.scale(12.0))) {
                mob.velocity = Vec3::new(facing.x * 0.6, 0.85, facing.z * 0.6);
                self.lunged = true;
                return;
            }
            self.breach = false;
        }
        if whale::can_spray(mob, cx.w) {
            self.skim += 1;
        }
        let mut y = self.surface - f64::from(mob.height()) + 0.1;
        let diff = y - mob.position.y;
        if diff.abs() < 1.0 {
            mob.velocity.y = (diff * 0.15).clamp(-0.06, 0.06);
            y = mob.position.y;
        }
        want(
            mob,
            at_height(mob.position.add_vector(facing.scale(10.0)), y),
        );
    }
}

#[derive(Clone, Debug, Default)]
struct Dive {
    next: u64,
    run: u16,
    bottom: u8,
}

fn floor(world: &MobWorld<'_>, position: Vec3) -> Option<f64> {
    let top = position.y.floor();
    (0..=32).map(|step| top - f64::from(step)).find(|y| {
        let point = at_height(position, *y);
        !whale::wet(world, point) && !is_air(world, point)
    })
}

impl Goal for Dive {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE
    }
    fn every_tick(&self) -> bool {
        true
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.fluids.in_water
            && u64::from(mob.age) >= self.next
            && floor(cx.w, mob.position).is_some_and(|ground| mob.position.y - ground >= 6.0)
    }
    fn can_continue(&mut self, mob: &mut Mob, _: &mut Cx) -> bool {
        self.run < 600
            && self.bottom < 150
            && !mob.horizontal_collision
            && !whale::grinding(mob)
            && mob.fluids.in_water
    }
    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        self.run = 0;
        self.bottom = 0;
        whale::motion_mut(&mut mob.ai.nav).diving = true;
        mob.nav_stop();
    }
    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        whale::motion_mut(&mut mob.ai.nav).diving = false;
        self.next = u64::from(mob.age) + 800 + mob.random.next_int_bound(800).unwrap_or(0) as u64;
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.run += 1;
        let ahead = mob
            .position
            .add_vector(whale::forward(mob.body_yaw).scale(10.0));
        let ground = floor(cx.w, mob.position)
            .into_iter()
            .chain(floor(cx.w, ahead))
            .reduce(f64::max);
        if let Some(ground) = ground {
            if mob.position.y - ground < 6.0 {
                self.bottom += 1;
            }
            want(mob, at_height(ahead, ground + 4.0));
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Swim {
    target: Vec3,
    last: Vec3,
    run: u16,
    stuck: u8,
}

fn open_water(world: &MobWorld<'_>, point: Vec3) -> bool {
    [
        (0.0, 0.0, 0.0),
        (0.0, 3.0, 0.0),
        (0.0, -1.0, 0.0),
        (2.0, 0.0, 0.0),
        (-2.0, 0.0, 0.0),
        (0.0, 0.0, 2.0),
        (0.0, 0.0, -2.0),
    ]
    .into_iter()
    .all(|(x, y, z)| whale::wet(world, offset(point, x, y, z)))
}

fn random_water(mob: &mut Mob, world: &MobWorld<'_>) -> Option<Vec3> {
    // BehaviorUtils.getRandomSwimmablePos resamples non-water path candidates at most ten times.
    let mut point = mob.default_random_pos(world, 24, 8);
    for _ in 0..10 {
        let Some([x, y, z]) = point else { break };
        let block = world.world.block(x.floor() as i32, y.floor() as i32, z.floor() as i32);
        if world.data.state(block).flags & super::super::data::WATER != 0 {
            break;
        }
        point = mob.default_random_pos(world, 24, 8);
    }
    point.map(|[x, y, z]| Vec3::new(x, y, z))
}

impl Goal for Swim {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        // The Java goal's reduced interval of one still advances the RNG.
        let _ = mob.random.next_int_bound(1);
        let mut target = None;
        if mob.random.next_float() < 0.7 {
            let forward = whale::forward(mob.body_yaw);
            target = (0..10).find_map(|_| {
                let turn = (mob.random.next_float() * 2.0 - 1.0) * 45.0 * DEG_TO_RAD;
                let reach = 14.0 + f64::from(mob.random.next_float()) * 8.0;
                let depth = -8.0 + f64::from(mob.random.next_float()) * 12.0;
                let point = mob.position.add_vector(forward.rotate_y(turn).scale(reach));
                let point = offset(point, 0.0, depth, 0.0);
                open_water(cx.w, point).then_some(point)
            });
        }
        let target = target.or_else(|| {
            (0..12).find_map(|_| random_water(mob, cx.w).filter(|point| open_water(cx.w, *point)))
        });
        let target = target.or_else(|| {
            random_water(mob, cx.w).map(|point| {
                if whale::wet(cx.w, offset(point, 0.0, 3.0, 0.0)) {
                    point
                } else {
                    offset(point, 0.0, -3.0, 0.0)
                }
            })
        });
        if let Some(target) = target {
            *self = Self {
                target,
                last: mob.position,
                run: 0,
                stuck: 0,
            };
            true
        } else {
            false
        }
    }
    fn can_continue(&mut self, mob: &mut Mob, _: &mut Cx) -> bool {
        self.run += 1;
        self.stuck = if self.last.subtract(mob.position).length_squared() < 0.0025 {
            self.stuck + 1
        } else {
            0
        };
        self.last = mob.position;
        self.run <= 300
            && self.stuck <= 40
            && mob.distance_sqr(self.target) >= 16.0
            && !mob.ai.nav.is_done()
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        mob.nav_move_to(cx.w, self.target.components(), 1.0);
    }
    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        mob.nav_stop();
    }
}
