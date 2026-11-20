use super::{
    Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_deer_variants::{appearance_texture, appearance_variant},
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};

thread_local! {
    static MODELS: RefCell<[ModelPart; 3]> = RefCell::new([
        bass_models::adult(), bass_models::medium(), bass_models::large(),
    ]);
    static POSES: RefCell<HashMap<u32, BassPose>> = RefCell::new(HashMap::new());
}

pub(super) struct BassPose {
    pub(super) clocks: [AnimationClock; 2],
    pub(super) age: i32,
    pub(super) tilt: [f32; 2],
    pub(super) pitch: [f32; 2],
    last_seen: Instant,
}

impl BassPose {
    pub(super) fn new(now: Instant) -> Self {
        Self { clocks: Default::default(), age: -1, tilt: [0.0; 2], pitch: [0.0; 2], last_seen: now }
    }

    pub(super) fn tick(&mut self, mob: &Mob, now: Instant) {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::new(now);
        }
        self.last_seen = now;
        if age == self.age {
            return;
        }
        self.age = age;

        let swimming = mob.fluids.in_water;
        self.clocks[0].advance_with_rate(!swimming, age, 0.5);
        self.clocks[1].advance_with_rate(swimming, age, 0.5);
        let [_, tilt] = self.tilt;
        let [_, pitch] = self.pitch;
        let mut next_tilt = 0.0;
        let mut pitch_target = 0.0;
        if swimming {
            let turn = (mob.previous_yaw - mob.yaw + 180.0).rem_euclid(360.0) - 180.0;
            next_tilt = if turn.abs() > 1.0 {
                if tilt.abs() < 25.0 { tilt - turn.signum() } else { tilt }
            } else {
                tilt.signum() * (tilt.abs() - 0.85).max(0.0)
            };
            let dx = mob.position.x - mob.previous.x;
            let dy = mob.position.y - mob.previous.y;
            let dz = mob.position.z - mob.previous.z;
            let horizontal = dx.hypot(dz);
            let speed = horizontal.hypot(dy);
            let weight = ((speed - 0.01) / 0.04).clamp(0.0, 1.0) as f32;
            pitch_target = -(dy.atan2(horizontal).to_degrees().clamp(-55.0, 55.0) as f32) * weight;
        }
        self.tilt = [tilt, next_tilt];
        self.pitch = [pitch, pitch + (pitch_target - pitch) * 0.2];
    }
}

pub(super) fn pose(poses: &mut HashMap<u32, BassPose>, id: u32, now: Instant) -> &mut BassPose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| BassPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_bass_animations(&self) {
        if self.remote.is_none() {
            return;
        }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| !mob.removed && mob.kind.id == "naturalist:bass") {
                pose(poses, mob.id, now).tick(mob, now);
            }
        });
    }

    pub(super) fn draw_naturalist_bass(&self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw) -> bool {
        let variant = appearance_variant(self, mob.id);
        let tier = match variant.as_deref() {
            Some("naturalist:bass_medium") => 1,
            Some("naturalist:bass_large") => 2,
            _ => 0,
        };
        let texture = appearance_texture(self, mob.id, false);
        let texture = texture.as_deref().unwrap_or("naturalist:entity/bass");
        MODELS.with_borrow_mut(|models| {
            let model = &mut models[tier];
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.tick(mob, now);
                }
                let gait = (render.walk_speed / 0.25).clamp(0.4, 2.0);
                use bass_animations::{ADULT_FLOP, ADULT_SWIM};
                pose.clocks[0].draw(&ADULT_FLOP, model, render.age, render.partial, 1.0, 1.0);
                pose.clocks[1].draw(&ADULT_SWIM, model, render.age, render.partial, gait, 1.0);
                let lerp = |[old, new]: [f32; 2]| old + (new - old) * render.partial;
                if let Some(body) = bone(model, if tier == 2 { "root" } else { "body" }) {
                    let pitch: f32 = lerp(pose.pitch);
                    body.x_rot += pitch.to_radians();
                }
                draw.rotate_about([0.0, 0.0, 1.0], -lerp(pose.tilt), [0.0; 3]);
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(draw, model, texture, if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF })
        })
    }
}
