use super::{Mob, MobEvent, MobWorld};
use horizon_core::geometry::{Axis, Vec3};

const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const PROBE: f64 = 0.06;
const GRACE: i32 = 12;
const WALL: f64 = 0.9;

#[derive(Clone, Debug)]
pub struct Surface {
    pub normal: Vec3,
    pub attached: bool,
    pub desired: Vec3,
    pub speed: f64,
    pub goal: Vec3,
    pub ground_cooldown: i32,
    grace: i32,
    switch_cooldown: i32,
    stuck: i32,
    nudge: Vec3,
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            normal: UP,
            attached: true,
            desired: Vec3::ZERO,
            speed: 0.0,
            goal: Vec3::ZERO,
            ground_cooldown: 0,
            grace: GRACE,
            switch_cooldown: 0,
            stuck: 0,
            nudge: Vec3::ZERO,
        }
    }
}

const AXES: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];

fn cardinal(axis: Axis, sign: f64) -> Vec3 {
    match axis {
        Axis::X => Vec3::new(sign, 0.0, 0.0),
        Axis::Y => Vec3::new(0.0, sign, 0.0),
        Axis::Z => Vec3::new(0.0, 0.0, sign),
    }
}

pub fn tangent(vector: Vec3, normal: Vec3) -> Vec3 {
    vector.subtract(normal.scale(vector.dot(normal)))
}

fn probe(mob: &Mob, world: &MobWorld<'_>, direction: Vec3) -> bool {
    let bounds = mob.bounds().deflate(Vec3::new(0.001, 0.001, 0.001));
    !world.no_collision(bounds.move_by(direction.scale(PROBE)))
}

impl Surface {
    pub fn on_side(&self) -> bool {
        self.attached && self.normal.y < WALL
    }

    pub fn halt(&mut self) {
        self.desired = Vec3::ZERO;
        self.speed = 0.0;
        self.goal = Vec3::ZERO;
    }

    fn attach(&mut self, mob: &mut Mob, normal: Vec3) {
        self.grace = GRACE;
        if !self.attached || self.normal != normal {
            self.normal = normal;
            self.attached = true;
            self.switch_cooldown = 5;
            mob.velocity = tangent(mob.velocity, normal);
        }
    }

    pub fn tick(&mut self, mob: &mut Mob) {
        if !self.attached && mob.on_ground && !mob.fluids.in_water && !mob.fluids.in_lava {
            self.attach(mob, UP);
        }
        if self.attached {
            mob.fall_distance = 0.0;
        }
    }

    pub fn travel(
        &mut self,
        mob: &mut Mob,
        world: &MobWorld<'_>,
        events: &mut Vec<MobEvent>,
    ) -> bool {
        if mob.fluids.in_water || mob.fluids.in_lava {
            self.attached = false;
        }
        if !self.attached {
            return false;
        }

        let velocity = mob
            .velocity
            .scale(0.6)
            .add_vector(self.desired.scale(self.speed * 0.16));
        let on_face = probe(mob, world, self.normal.reverse());
        let press = on_face || self.normal.y > 0.5 || self.goal.dot(self.normal) < 0.0;
        let mut intended = if press {
            velocity.subtract(self.normal.scale(0.08))
        } else {
            velocity
        };
        if self.stuck > 2 && self.normal.y < WALL {
            if self.stuck == 3 {
                let cross = self.normal.cross(self.desired);
                if cross.length_squared() > 1.0e-6 {
                    let side = cross.normalize();
                    self.nudge = if world.no_collision(mob.bounds().move_by(side.scale(0.35))) {
                        side.scale(0.05)
                    } else if world.no_collision(mob.bounds().move_by(side.scale(-0.35))) {
                        side.scale(-0.05)
                    } else {
                        Vec3::ZERO
                    };
                }
            }
            if self.nudge.length_squared() > 1.0e-8 {
                intended = intended.add_vector(self.nudge);
            }
        }

        let start = mob.position;
        mob.velocity = intended;
        mob.do_move(world, events);
        let actual = mob.position.subtract(start);
        mob.velocity = velocity;
        self.update_attachment(mob, world, intended, actual);
        self.switch_cooldown = (self.switch_cooldown - 1).max(0);
        self.ground_cooldown = (self.ground_cooldown - 1).max(0);
        self.update_stuck(actual);
        true
    }

    fn update_attachment(
        &mut self,
        mob: &mut Mob,
        world: &MobWorld<'_>,
        intended: Vec3,
        actual: Vec3,
    ) {
        if let Some(face) = self.steered_face(mob, world, intended, actual) {
            self.attach(mob, face);
            return;
        }
        if probe(mob, world, self.normal.reverse()) {
            self.grace = GRACE;
            return;
        }
        if let Some(face) = self.nearest_face(mob, world)
            && (self.ground_cooldown <= 0 || face.y > WALL)
        {
            self.attach(mob, face);
            return;
        }
        self.grace -= 1;
        if self.grace <= 0 {
            self.attached = false;
        }
    }

    fn steered_face(
        &self,
        mob: &Mob,
        world: &MobWorld<'_>,
        intended: Vec3,
        actual: Vec3,
    ) -> Option<Vec3> {
        if self.switch_cooldown > 0 || self.ground_cooldown > 0 {
            return None;
        }
        let mut best = None;
        let mut best_weight = 0.0;
        for axis in AXES {
            let wanted = intended.component(axis);
            let weight = self.desired.component(axis).abs();
            if (wanted - actual.component(axis)).abs() < 1.0e-7
                || wanted.abs() < 1.0e-7
                || weight < 0.05
                || weight <= best_weight
            {
                continue;
            }
            let face = cardinal(axis, -wanted.signum());
            if face.dot(self.normal) > 0.5 {
                continue;
            }
            if self.normal.y > WALL && face.y == 0.0 {
                let step = Vec3::new(
                    -face.x * PROBE,
                    mob.kind.step_height + 0.001,
                    -face.z * PROBE,
                );
                if world.no_collision(
                    mob.bounds()
                        .deflate(Vec3::new(0.001, 0.001, 0.001))
                        .move_by(step),
                ) {
                    continue;
                }
            }
            if probe(mob, world, face.reverse()) {
                best = Some(face);
                best_weight = weight;
            }
        }
        best
    }

    fn nearest_face(&self, mob: &Mob, world: &MobWorld<'_>) -> Option<Vec3> {
        let mut best = None;
        let mut best_dot = -0.5;
        for axis in AXES {
            for sign in [-1.0, 1.0] {
                let face = cardinal(axis, sign);
                if face == self.normal || !probe(mob, world, face.reverse()) {
                    continue;
                }
                let dot = face.dot(self.normal);
                if dot > best_dot {
                    best = Some(face);
                    best_dot = dot;
                }
            }
        }
        best
    }

    fn update_stuck(&mut self, actual: Vec3) {
        if self.speed <= 1.0e-3 {
            self.stuck = 0;
            self.nudge = Vec3::ZERO;
            return;
        }
        if self.desired.length_squared() > 1.0e-4 && actual.dot(self.desired) > 1.0e-4 {
            self.stuck = 0;
            if self.nudge.length_squared() < 1.0e-8 || actual.dot(self.desired) > 0.005 {
                self.nudge = Vec3::ZERO;
            }
            return;
        }
        if actual.length_squared() > 1.0e-6 && self.desired.length_squared() <= 1.0e-4 {
            self.stuck = 0;
            self.nudge = Vec3::ZERO;
            return;
        }
        self.stuck += 1;
        if self.normal.y <= WALL && self.stuck > 10 {
            self.stuck = 0;
            self.nudge = Vec3::ZERO;
            self.attached = false;
        }
    }
}
