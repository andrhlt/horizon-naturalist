use super::super::{Mob, MobEvent, MobWorld, living::wrap};
use super::{Navigation, naturalist_shark_state};
use horizon_core::{
    geometry::{Aabb, Vec3},
    random::RandomSource,
};

pub const KIND: &str = "naturalist:whale";
const PARTS: [(f64, f32, f32); 5] = [
    (4.0, 2.2, 2.2),
    (1.5, 3.0, 2.5),
    (-1.0, 3.0, 2.5),
    (-3.5, 2.2, 2.0),
    (-5.3, 1.4, 1.2),
];

#[derive(Clone, Debug, Default)]
pub struct Motion {
    initialized: bool,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub fin_lag: f32,
    pub segment_yaw: [f32; 4],
    pub segment_pitch: [f32; 4],
    pub front_droop: f32,
    pub back_droop: f32,
    pub body_push_ticks: u8,
    pub contact_position: Option<Vec3>,
    pub spray_cooldown: i32,
    pub flop_cooldown: i32,
    pub diving: bool,
}

impl Motion {
    pub fn tick(&mut self, mob: &Mob, is_water: impl Fn(Vec3) -> bool) {
        let previous_parts = parts(mob, self);
        let wet = mob.fluids.in_water;
        if !self.initialized {
            self.initialized = true;
            self.yaw = mob.body_yaw;
            self.segment_yaw.fill(mob.body_yaw);
        }
        let previous_yaw = self.yaw;
        self.yaw += wrap(mob.body_yaw - self.yaw) * 0.15;
        let grounded = mob.on_ground || mob.vertical_collision_below;
        let horizontal = mob.velocity.horizontal_distance_squared();
        let pitch =
            if !grounded && horizontal > 1.0e-7 && (wet || mob.velocity.length_squared() > 0.03) {
                let limit = if wet { 40.0 } else { 60.0 };
                (-mob.velocity.y.atan2(horizontal.sqrt()).to_degrees() as f32).clamp(-limit, limit)
            } else {
                0.0
            };
        self.pitch += (pitch - self.pitch) * if grounded { 0.25 } else { 0.07 };
        self.fin_lag += (self.pitch - self.fin_lag) * 0.1;
        for index in 0_usize..4 {
            let parent = index.saturating_sub(1);
            let yaw = [
                mob.body_yaw,
                self.yaw,
                self.segment_yaw[parent],
                self.segment_yaw[parent],
            ][index];
            let target_pitch = [
                pitch,
                self.pitch,
                self.segment_pitch[parent],
                self.segment_pitch[parent],
            ][index];
            self.segment_yaw[index] +=
                wrap(yaw - self.segment_yaw[index]) * [0.35, 0.14, 0.11, 0.09][index];
            self.segment_pitch[index] +=
                (target_pitch - self.segment_pitch[index]) * [0.24, 0.12, 0.10, 0.08][index];
        }
        let roll = (-wrap(self.yaw - previous_yaw) * 5.0).clamp(-30.0, 30.0);
        self.roll += (roll - self.roll) * 0.06;
        let out_of_water = |part: Aabb| {
            if wet && !is_water(part.center()) {
                1.0
            } else {
                0.0
            }
        };
        self.front_droop += (out_of_water(previous_parts[0]) - self.front_droop) * 0.08;
        self.back_droop += (out_of_water(previous_parts[3]) - self.back_droop) * 0.08;
    }
}

pub fn motion(nav: &Navigation) -> Option<&Motion> {
    nav.__horizon_mixins_naturalist_whale_motion().as_deref()
}

pub fn motion_mut(nav: &mut Navigation) -> &mut Motion {
    nav.__horizon_mixins_naturalist_whale_motion_mut()
        .get_or_insert_with(Box::default)
}

pub fn parts(mob: &Mob, state: &Motion) -> [Aabb; 5] {
    parts_at(mob, state, mob.position)
}

pub fn contact_parts(mob: &Mob) -> Option<[Aabb; 5]> {
    let state = motion(&mob.ai.nav)?;
    Some(parts_at(mob, state, state.contact_position?))
}

fn parts_at(mob: &Mob, state: &Motion, position: Vec3) -> [Aabb; 5] {
    let scale = if mob.is_baby() { 0.5 } else { 1.0 };
    let yaw = if state.initialized {
        state.yaw
    } else {
        mob.body_yaw
    };
    let (sin_yaw, cos_yaw) = naturalist_shark_state::part_angles(yaw);
    let (sin_pitch, cos_pitch) = naturalist_shark_state::part_angles(state.pitch);
    PARTS.map(|(raw, width, height)| {
        let offset = raw * scale;
        let half = f64::from(width) * 0.5 * scale;
        let height = height * scale as f32;
        let (droop, end) = if raw > 0.0 {
            (state.front_droop, 4.0)
        } else {
            (state.back_droop, -5.3)
        };
        let ratio = raw / end;
        let x = position.x - sin_yaw * offset * cos_pitch;
        let y = position.y + f64::from(mob.height() - height) * 0.5 - sin_pitch * offset;
        let y = y - f64::from(droop) * ratio * ratio * 0.9 * scale;
        let z = position.z + cos_yaw * offset * cos_pitch;
        Aabb::new(
            Vec3::new(x - half, y, z - half),
            Vec3::new(x + half, y + f64::from(height), z + half),
        )
    })
}

pub(in crate::mobs) fn wet(world: &MobWorld<'_>, at: Vec3) -> bool {
    let [x, y, z] = at.components().map(|coordinate| coordinate.floor() as i32);
    world.fluid(x, y, z) == crate::blocks::Fluid::Water
}

pub(in crate::mobs) fn forward(yaw: f32) -> Vec3 {
    Vec3::direction_from_rotation(0.0, yaw)
}

pub(in crate::mobs) fn can_spray(mob: &Mob, world: &MobWorld<'_>) -> bool {
    let [x, y, z] = mob
        .position
        .add_vector(Vec3::new(0.0, f64::from(mob.height()) + 1.0, 0.0))
        .components()
        .map(|coordinate| coordinate.floor() as i32);
    mob.fluids.in_water && world.fluid(x, y, z) == crate::blocks::Fluid::None
}

pub(in crate::mobs) fn grinding(mob: &Mob) -> bool {
    motion(&mob.ai.nav).is_some_and(|state| state.body_push_ticks >= 20)
}

pub(in crate::mobs) fn tick_body(mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) {
    let mut state = motion(&mob.ai.nav).cloned().unwrap_or_default();
    state.tick(mob, |position| wet(world, position));
    state.contact_position = Some(mob.position);
    let boxes = parts(mob, &state);
    if naturalist_shark_state::push_parts(mob, boxes, world, events) {
        state.body_push_ticks = state.body_push_ticks.saturating_add(2).min(40);
    } else {
        state.body_push_ticks = state.body_push_ticks.saturating_sub(1);
    }
    *motion_mut(&mut mob.ai.nav) = state;
}

pub(in crate::mobs) fn tick_spray(mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) {
    let state = motion_mut(&mut mob.ai.nav);
    if state.spray_cooldown > 0 {
        state.spray_cooldown -= 1;
        return;
    }
    if !can_spray(mob, world) {
        return;
    }
    let reach = if mob.is_baby() { 1.0 } else { 2.0 };
    let heading = Vec3::direction_from_rotation(mob.head_pitch, mob.yaw);
    let at = mob
        .position
        .add_vector(Vec3::new(
            heading.x * reach,
            f64::from(mob.height()) * 0.9,
            heading.z * reach,
        ))
        .components();
    events.push(MobEvent::Particles {
        kind: "naturalist:whale_cloud",
        at,
        count: 14,
    });
    events.push(MobEvent::Particles {
        kind: "naturalist:whale_bubbles",
        at,
        count: 8,
    });
    if !mob.silent {
        events.push(MobEvent::SoundId(
            "naturalist:entity.whale.blowhole".into(),
            mob.position.components(),
            1.2,
            1.0,
        ));
    }
    motion_mut(&mut mob.ai.nav).spray_cooldown = 160 + mob.random.next_int_bound(160).unwrap_or(0);
}
