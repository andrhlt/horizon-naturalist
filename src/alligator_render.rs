use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::appearance_texture,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::{
    cell::RefCell,
    collections::HashMap,
    time::{Duration, Instant},
};

#[derive(Clone, Copy)]
struct Chain {
    yaw: f32,
    pitch: f32,
    roll: f32,
    tails_yaw: [f32; 3],
    tails_pitch: [f32; 3],
}
impl Chain {
    fn new(yaw: f32) -> Self {
        Self {
            yaw,
            pitch: 0.0,
            roll: 0.0,
            tails_yaw: [yaw; 3],
            tails_pitch: [0.0; 3],
        }
    }
    fn tick(
        &mut self,
        body_yaw: f32,
        target_pitch: f32,
        grounded: bool,
        swimming: bool,
        baby: bool,
    ) {
        let yaw_step = wrap(body_yaw - self.yaw);
        self.yaw += yaw_step;
        self.pitch += (target_pitch - self.pitch) * if grounded { 0.25 } else { 0.07 };
        let roll = if swimming {
            (-yaw_step * 4.0).clamp(-25.0, 25.0)
        } else {
            0.0
        };
        self.roll += (roll - self.roll) * 0.1;
        let (yaw_gains, pitch_gains) = if baby {
            ([0.55, 0.28, 0.22], [0.4, 0.22, 0.18])
        } else {
            ([0.35, 0.16, 0.12], [0.24, 0.12, 0.1])
        };
        for i in 0..3 {
            let yaw = if i <= 1 {
                self.yaw
            } else {
                self.tails_yaw[i - 1]
            };
            let pitch = match i {
                0 => target_pitch,
                1 => self.pitch,
                _ => self.tails_pitch[i - 1],
            };
            self.tails_yaw[i] += wrap(yaw - self.tails_yaw[i]) * yaw_gains[i];
            self.tails_pitch[i] += (pitch - self.tails_pitch[i]) * pitch_gains[i];
        }
    }
}

fn wrap(value: f32) -> f32 {
    (value + 180.0).rem_euclid(360.0) - 180.0
}
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
fn rot_lerp(a: f32, b: f32, t: f32) -> f32 {
    a + wrap(b - a) * t
}

struct Pose {
    clocks: [AnimationClock; 4],
    previous: Chain,
    current: Chain,
    tick: i32,
    last_seen: Instant,
}

thread_local! {
    static MODELS: RefCell<[ModelPart; 2]> = RefCell::new([alligator_models::adult(), alligator_models::baby()]);
    static POSES: RefCell<HashMap<u32, Pose>> = RefCell::new(HashMap::new());
}

impl Game<'_> {
    pub(super) fn draw_naturalist_alligator(
        &self,
        mob: &Mob,
        r: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        use alligator_animations::*;
        let texture = appearance_texture(self, r.id, r.baby);
        let (fallback, glow, idle, swim, walk) = if r.baby {
            (
                "naturalist:entity/alligator/alligator_baby",
                "naturalist:entity/alligator/alligator_baby_glowmask",
                &BABY_IDLE,
                &BABY_SWIM,
                &BABY_WALK,
            )
        } else {
            (
                "naturalist:entity/alligator/alligator",
                "naturalist:entity/alligator/alligator_glowmask",
                &ADULT_IDLE,
                &ADULT_SWIM,
                &ADULT_WALK,
            )
        };
        MODELS.with_borrow_mut(|models| {
            let model = &mut models[usize::from(r.baby)];
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let tick = r.age.floor() as i32;
                if poses.len() > 1024 {
                    poses.retain(|_, pose| {
                        now.duration_since(pose.last_seen) < Duration::from_secs(30)
                    });
                }
                let new = || Pose {
                    clocks: Default::default(),
                    previous: Chain::new(mob.body_yaw),
                    current: Chain::new(mob.body_yaw),
                    tick: -1,
                    last_seen: now,
                };
                let pose = poses.entry(r.id).or_insert_with(new);
                if tick < pose.tick {
                    *pose = new();
                }
                pose.last_seen = now;
                if tick != pose.tick {
                    let swimming = mob.fluids.in_water;
                    let grounded = !swimming || r.on_ground;
                    let movement = mob.position.subtract(mob.previous);
                    let horizontal = movement.x.hypot(movement.z);
                    let pitch = if !grounded && horizontal * horizontal > 1.0e-7 {
                        (-(movement.y.atan2(horizontal).to_degrees() as f32)).clamp(-35.0, 35.0)
                    } else {
                        0.0
                    };
                    pose.previous = pose.current;
                    pose.current
                        .tick(mob.body_yaw, pitch, grounded, swimming, r.baby);
                    let walking = !swimming
                        && (mob.velocity.x * mob.velocity.x + mob.velocity.z * mob.velocity.z)
                            > 1.0e-6;
                    for (index, (clock, active)) in pose
                        .clocks
                        .iter_mut()
                        .zip([
                            mob.ai.swinging && !r.baby,
                            !swimming && !walking,
                            swimming,
                            walking,
                        ])
                        .enumerate()
                    {
                        clock.advance_with_rate(active, tick, if index == 0 { 1.0 } else { 0.5 });
                    }
                    pose.tick = tick;
                }
                let gait = (8.64 * mob.movement_speed()).min(1.0) as f32 * 0.65;
                for (clock, (clip, rate, weight)) in pose.clocks.iter_mut().zip([
                    (&ADULT_BITE, 1.0, 1.0),
                    (idle, 0.6, 1.0 - (r.walk_speed * 2.5).min(1.0)),
                    (
                        swim,
                        (r.walk_speed / 0.5).clamp(0.1, 2.0) * if r.baby { 5.0 } else { 1.0 },
                        1.0,
                    ),
                    (
                        walk,
                        (r.walk_speed / gait.max(0.05)).clamp(0.4, 2.0)
                            * if r.baby { 5.0 } else { 3.0 },
                        1.0,
                    ),
                ]) {
                    clock.draw(clip, model, r.age, r.partial, rate, weight);
                }
                let (a, b, t) = (pose.previous, pose.current, r.partial);
                let pitch = lerp(a.pitch, b.pitch, t);
                let roll = lerp(a.roll, b.roll, t);
                let limbs = if r.baby {
                    ["leftArm", "rightArm", "leftLeg", "rightLeg"]
                } else {
                    ["left_arm", "right_arm", "left_leg", "right_leg"]
                };
                if let Some(body) = bone(model, "body") {
                    body.x_rot += pitch.to_radians();
                    body.z_rot += roll.to_radians();
                }
                if let Some(neck) = bone(model, "neck") {
                    neck.x_rot += (-pitch * 0.5 + r.pitch).to_radians();
                    neck.y_rot += r.head_yaw.to_radians();
                    neck.z_rot += (-roll * 0.5).to_radians();
                }
                for (i, key) in [(1, "tail"), (2, if r.baby { "tailTip" } else { "tail2" })] {
                    if let Some(tail) = bone(model, key) {
                        let yaw_ref = if i == 1 {
                            rot_lerp(a.yaw, b.yaw, t)
                        } else {
                            rot_lerp(a.tails_yaw[i - 1], b.tails_yaw[i - 1], t)
                        };
                        let pitch_ref = if i == 1 {
                            pitch
                        } else {
                            lerp(a.tails_pitch[i - 1], b.tails_pitch[i - 1], t)
                        };
                        tail.y_rot += wrap(rot_lerp(a.tails_yaw[i], b.tails_yaw[i], t) - yaw_ref)
                            .to_radians();
                        tail.x_rot +=
                            (lerp(a.tails_pitch[i], b.tails_pitch[i], t) - pitch_ref).to_radians();
                    }
                }
                let lift = pitch.to_radians().sin() * if r.baby { 3.5 } else { 11.0 };
                for (index, key) in limbs.into_iter().enumerate() {
                    if let Some(limb) = bone(model, key) {
                        limb.x_rot += (pitch * 0.35).to_radians();
                        limb.z_rot += (-roll * 0.35).to_radians();
                        limb.y += if index < 2 { lift } else { -lift };
                    }
                }
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let rendered = self.native_draw_model_part(
                draw,
                model,
                texture.as_deref().unwrap_or(fallback),
                if r.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF },
            );
            let original = draw.light;
            draw.light = 240 | (240 << 8);
            self.native_draw_model_part(draw, model, glow, 0xFFFFFF);
            draw.light = original;
            rendered
        })
    }
}
