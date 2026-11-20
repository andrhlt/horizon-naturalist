use super::{
    super::{Mob, MobEvent, MobWorld, Who},
    Navigation,
};
use horizon_core::{geometry::{Aabb, Vec3}, random::RandomSource};

pub const KIND: &str = "naturalist:great_white_shark";
pub const PREY: &str = "naturalist:great_white_shark_hostiles";
const PARTS: [(f64, f32, f32); 3] = [(2.0, 1.2, 1.2), (-1.5, 1.0, 1.0), (-2.9, 0.8, 1.2)];

#[derive(Clone, Debug, Default)]
pub struct Motion {
    initialized: bool,
    pub flop_cooldown: i32,
    pub contact_position: Option<Vec3>,
    pub yaw: f32,
    pub previous_yaw: f32,
    pub pitch: f32,
    pub previous_pitch: f32,
    pub roll: f32,
    pub previous_roll: f32,
    pub segment_yaw: [f32; 3],
    pub previous_segment_yaw: [f32; 3],
    pub segment_pitch: [f32; 3],
    pub previous_segment_pitch: [f32; 3],
}

fn wrap(value: f32) -> f32 {
    super::super::living::wrap(value)
}

impl Motion {
    pub fn tick(&mut self, yaw: f32, velocity: Vec3, wet: bool) {
        if !self.initialized {
            self.initialized = true;
            self.yaw = yaw;
            self.segment_yaw.fill(yaw);
        }
        self.previous_yaw = self.yaw;
        self.previous_pitch = self.pitch;
        self.previous_roll = self.roll;
        self.previous_segment_yaw = self.segment_yaw;
        self.previous_segment_pitch = self.segment_pitch;

        let horizontal = velocity.horizontal_distance_squared().sqrt();
        let wanted_pitch = if wet && horizontal > 0.01 {
            (-velocity.y.atan2(horizontal).to_degrees() as f32).clamp(-40.0, 40.0)
        } else {
            0.0
        };
        self.pitch += (wanted_pitch - self.pitch) * 0.1;
        self.yaw += wrap(yaw - self.yaw) * 0.2;
        for index in 0..3 {
            let yaw_reference = [yaw, self.yaw, self.segment_yaw[1]][index];
            let pitch_reference = [wanted_pitch, self.pitch, self.segment_pitch[1]][index];
            self.segment_yaw[index] +=
                wrap(yaw_reference - self.segment_yaw[index]) * [0.35, 0.16, 0.12][index];
            self.segment_pitch[index] +=
                (pitch_reference - self.segment_pitch[index]) * [0.24, 0.12, 0.1][index];
        }
        let roll_target = (-wrap(self.yaw - self.previous_yaw) * 4.0).clamp(-25.0, 25.0);
        self.roll += (roll_target - self.roll) * 0.1;
    }
}

pub fn motion(nav: &Navigation) -> Option<&Motion> {
    nav.__horizon_mixins_naturalist_shark_motion().as_deref()
}

pub fn motion_mut(nav: &mut Navigation) -> &mut Motion {
    nav.__horizon_mixins_naturalist_shark_motion_mut()
        .get_or_insert_with(Box::default)
}

pub fn is_prey(nav: &Navigation) -> bool {
    *nav.__horizon_mixins_naturalist_shark_prey()
}

pub fn set_prey(nav: &mut Navigation, value: bool) {
    *nav.__horizon_mixins_naturalist_shark_prey_mut() = value;
}

pub(super) fn part_angles(degrees: f32) -> (f64, f64) {
    use horizon_core::geometry::trig::{DEG_TO_RAD, Mth};
    let angle = degrees * DEG_TO_RAD;
    (f64::from(angle.msin()), f64::from(angle.mcos()))
}

/// Each part uses the parent's smoothed yaw and pitch, with its own vertical center.
pub fn parts(position: Vec3, parent_height: f64, yaw: f32, pitch: f32) -> [Aabb; 3] {
    let (sin_yaw, cos_yaw) = part_angles(yaw);
    let (sin_pitch, cos_pitch) = part_angles(pitch);
    PARTS.map(|(z, width, height)| {
        let x = position.x - sin_yaw * z * cos_pitch;
        let y = position.y + f64::from(parent_height as f32 - height) * 0.5 - sin_pitch * z;
        let z = position.z + cos_yaw * z * cos_pitch;
        let half = f64::from(width) * 0.5;
        Aabb::new(
            Vec3::new(x - half, y, z - half),
            Vec3::new(x + half, y + f64::from(height), z + half),
        )
    })
}

pub(in crate::mobs) fn nearest_water(mob: &Mob, world: &MobWorld<'_>) -> Option<Vec3> {
    use crate::blocks::Fluid;

    let [x, y, z] = mob.position.components().map(|part| part.floor() as i32);
    if !world
        .world
        .may_hold([x - 16, y - 8, z - 16], [x + 16, y + 1, z + 16], |state| {
            world.facts.get(state).fluid == Fluid::Water
        })
    {
        return None;
    }
    let mut nearest = None;
    let mut closest = i32::MAX;
    for dz in -16_i32..=16 {
        for dy in -8_i32..=1 {
            for dx in -16_i32..=16 {
                let distance = dx * dx + dy * dy + dz * dz;
                if distance < closest && world.fluid(x + dx, y + dy, z + dz) == Fluid::Water {
                    closest = distance;
                    nearest = Some((x + dx, z + dz));
                }
            }
        }
    }
    let (x, z) = nearest?;
    let delta = Vec3::new(
        f64::from(x) + 0.5 - mob.position.x,
        0.0,
        f64::from(z) + 0.5 - mob.position.z,
    );
    (delta.length_squared() >= 1.0e-4).then(|| delta.normalize())
}

pub(in crate::mobs) fn move_parts(mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) {
    motion_mut(&mut mob.ai.nav).contact_position = Some(mob.position);
    let state = motion(&mob.ai.nav).expect("shark body motion has ticked");
    let parts = parts(
        mob.position,
        f64::from(mob.height()),
        state.yaw,
        state.pitch,
    );
    push_parts(mob, parts, world, events);
}

pub(super) fn push_parts<const N: usize>(
    mob: &mut Mob, parts: [Aabb; N], world: &MobWorld<'_>, events: &mut Vec<MobEvent>,
) -> bool {
    let center = mob.bounds().center();
    let mut push = Vec3::ZERO;
    for part in parts {
        let part_center = part.center();
        if !world.no_collision(part) {
            let away = center.subtract(part_center);
            if away.length_squared() >= 1.0e-4 {
                push = push.add_vector(away.normalize().scale(0.04));
            }
        }
        for other in world.others_near(part).filter(|other| {
            other.id != mob.id
                && other.alive
                && other.pushable
                && !other.passenger
                && !other.sleeping
                && !other.is_vehicle
                && part.intersects(other.bounds)
        }) {
            shark_part_push(Who::Mob(other.id), other.position, part_center, events);
        }
    }
    if push.length_squared() > 0.0 {
        if push.length_squared() > 0.12 * 0.12 {
            push = push.normalize().scale(0.12);
        }
        let velocity = mob.velocity;
        mob.velocity = push;
        mob.do_move(world, events);
        mob.velocity = velocity.add_vector(push.scale(0.2));
        return true;
    }
    false
}

fn shark_part_push(who: Who, target: Vec3, part: Vec3, events: &mut Vec<MobEvent>) {
    if let Some(velocity) = part_impulse(target, part) {
        events.push(MobEvent::Push {
            target: who,
            velocity: velocity.components(),
        });
    }
}

pub fn part_impulse(target: Vec3, part: Vec3) -> Option<Vec3> {
    let delta = target.subtract(part);
    let maximum = delta.x.abs().max(delta.z.abs());
    if maximum < f64::from(0.01_f32) {
        return None;
    }
    let root = maximum.sqrt();
    let scale = (1.0 / root).min(1.0);
    let strength = f64::from(0.05_f32);
    Some(Vec3::new(delta.x / root * scale * strength, 0.0, delta.z / root * scale * strength))
}

pub(in crate::mobs) fn tick_flop(mob: &mut Mob, world: &MobWorld<'_>, cooldown: i32) -> (i32, bool) {
    if mob.fluids.in_water || !mob.on_ground || !mob.vertical_collision_below {
        return (cooldown, false);
    }
    if cooldown > 0 {
        return (cooldown - 1, false);
    }
    let drift = match nearest_water(mob, world) {
        Some(direction) => {
            let movement = direction.rotate_y((mob.random.next_float() - 0.5) * 0.35);
            let yaw = (movement.z.atan2(movement.x).to_degrees() - 90.0) as f32;
            mob.yaw = yaw;
            mob.body_yaw = yaw;
            mob.head_yaw = yaw;
            movement.scale(0.45)
        }
        None => {
            let x = f64::from(mob.random.next_float() * 2.0 - 1.0) * 0.3;
            let z = f64::from(mob.random.next_float() * 2.0 - 1.0) * 0.3;
            Vec3::new(x, 0.0, z)
        }
    };
    mob.velocity = mob.velocity.add_vector(Vec3::new(drift.x, 0.5, drift.z));
    mob.on_ground = false;
    (15 + mob.random.next_int_bound(25).unwrap_or(0), true)
}
