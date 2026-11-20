use super::{Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_deer_variants::crab_appearance, naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone}};
use horizon_assets::item_model::DisplayContext;
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use horizon_core::random::{LegacyRandom, RandomSource};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};

struct Pose {
    clocks: [AnimationClock; 9],
    tick: i32,
    last_seen: Instant,
    hide_start: i32,
    unhide_end: i32,
    peek_start: i32,
    peek_end: i32,
    hiding: bool,
    random: LegacyRandom,
    last_walk_phase: Option<i64>,
}

impl Pose {
    fn new(id: u32, now: Instant) -> Self {
        Self {
            clocks: Default::default(), tick: -1, last_seen: now,
            hide_start: -1, unhide_end: -1, peek_start: -1, peek_end: -1,
            hiding: false, random: LegacyRandom::new(i64::from(id)), last_walk_phase: None,
        }
    }

    fn step_cues(&mut self) -> usize {
        let current = self.clocks[1].started_phase_ms();
        let previous = std::mem::replace(&mut self.last_walk_phase, current);
        let Some(now) = current else { return 0; };
        let previous = previous.filter(|old| *old <= now).unwrap_or(-1);
        [0, 250].into_iter().filter(|cue| {
            (now - cue).div_euclid(500) > (previous - cue).div_euclid(500)
        }).count()
    }

    fn tick(&mut self, tick: i32, bits: u8, moving: bool) {
        let hiding = bits & 1 != 0;
        if hiding {
            if !self.hiding {
                self.hide_start = tick;
                self.peek_start = tick + 87;
                self.peek_end = self.peek_start + 23;
            }
            if tick >= self.peek_end {
                self.peek_start = tick + 60 + self.random.next_int_bound(100).unwrap_or(0);
                self.peek_end = self.peek_start + 23;
            }
            self.unhide_end = -1;
        } else if self.hiding {
            self.unhide_end = tick + 9;
        }
        let unhide = !hiding && tick < self.unhide_end;
        let peeking = hiding && tick >= self.peek_start && tick < self.peek_end;
        let hide_loop = hiding && !peeking && tick - self.hide_start >= 38;
        let dancing = !hiding && !unhide && bits & 2 != 0;
        let sitting = !hiding && !unhide && !dancing && bits & 4 != 0;
        let idle = !hiding && !unhide && !dancing && !sitting;
        for (index, (clock, active)) in self.clocks.iter_mut().zip([
            idle && !moving, idle && moving, hide_loop, sitting, hiding && !peeking && !hide_loop,
            unhide, bits & 8 != 0 && !dancing, dancing, peeking,
        ]).enumerate() {
            clock.advance_with_rate(active, tick, if index == 6 { 1.0 } else { 0.5 });
        }
        self.tick = tick;
        self.hiding = hiding;
    }
}

thread_local! {
    static MODELS: RefCell<[ModelPart; 2]> = RefCell::new([crab_models::adult(), crab_models::baby()]);
    static POSES: RefCell<HashMap<u32, Pose>> = RefCell::new(HashMap::new());
}

impl Game<'_> {
    pub(super) fn tick_naturalist_crab_steps(&mut self) {
        if self.remote.is_none() { return; }
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| !mob.removed && mob.kind.id == "naturalist:crab") {
                if let Some(pose) = poses.get_mut(&mob.id) {
                    for _ in 0..pose.step_cues() {
                        self.audio.play_unshared("naturalist:entity.crab.step", Some(mob.position.components()), 0.3, 1.0);
                    }
                }
            }
        });
    }

    pub(super) fn draw_naturalist_crab(&self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw) -> bool {
        use crab_animations::*;
        let (texture, bits) = crab_appearance(self, mob, render.baby);
        MODELS.with_borrow_mut(|models| {
            let model = &mut models[usize::from(render.baby)];
            model.reset_all_poses();
            if let Some(root) = bone(model, "root") {
                root.y_rot += std::f32::consts::FRAC_PI_2;
            }
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let tick = render.age.floor() as i32;
                if poses.len() > 2048 {
                    poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
                }
                let pose = poses.entry(render.id).or_insert_with(|| Pose::new(render.id, now));
                if tick < pose.tick { *pose = Pose::new(render.id, now); }
                pose.last_seen = now;
                if pose.tick != tick {
                    pose.tick(tick, bits, render.walk_speed > 0.001);
                }
                if render.baby {
                    for (index, clip, speed) in [(0, &BABY_IDLE, 1.0),
                        (1, &BABY_WALK, (render.walk_speed * 6.0).min(1.0)), (3, &BABY_SIT_IDLE, 1.0)]
                    {
                        pose.clocks[index].draw(clip, model, render.age, render.partial, speed, 1.0);
                    }
                } else {
                    for (clock, (clip, speed)) in pose.clocks.iter_mut().zip([
                        (&ADULT_IDLE, 1.0), (&ADULT_WALK, (render.walk_speed * 4.0).min(1.0)),
                        (&ADULT_HIDE_LOOP, 1.0), (&ADULT_SIT_LOOP, 1.0), (&ADULT_HIDE, 1.0),
                        (&ADULT_UNHIDE, 1.0), (&ADULT_SWING_RIGHT, 1.0), (&ADULT_DANCE, 1.0), (&ADULT_HIDE_PEEK, 1.0),
                    ]) {
                        clock.draw(clip, model, render.age, render.partial, speed, 1.0);
                    }
                }
            });
            let to_camera = -draw.pose().w_axis.truncate();
            let forward = render.body_yaw.to_radians() + std::f32::consts::FRAC_PI_2;
            let distance = to_camera.x.hypot(to_camera.z);
            let camera_yaw = if distance < 1.0e-4 { forward } else { (-to_camera.x).atan2(to_camera.z) };
            let yaw = (forward - camera_yaw).to_degrees();
            let yaw = ((yaw + 180.0).rem_euclid(360.0) - 180.0).clamp(
                if render.baby { -22.5 } else { -90.0 }, if render.baby { 22.5 } else { 90.0 });
            let limit = if render.baby { 10.0 } else { 22.5_f32 };
            let pitch = to_camera.y.atan2(distance).clamp(-limit.to_radians(), limit.to_radians());
            for name in ["leftEye", "rightEye"] {
                if let Some(eye) = bone(model, name) {
                    eye.x_rot -= pitch;
                    eye.y_rot -= yaw.to_radians();
                }
            }
            draw.push();
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let drawn = self.native_draw_model_part(draw, model, &texture, if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF });
            if !render.baby && let Some(stack) = mob.extra.main_hand_stack() {
                draw.push();
                if claw_pose(draw, model) {
                    draw.scale([-1.0, -1.0, 1.0]);
                    self.native_draw_item(draw, stack, DisplayContext::ThirdPersonRightHand, None, mob.id as i32);
                }
                draw.pop();
            }
            draw.pop();
            drawn
        })
    }
}

fn claw_pose(draw: &mut BlockEntityDraw, model: &ModelPart) -> bool {
    let mut part = model;
    for name in ["root", "head", "crabBody", "rightClaw", "rightArm", "rightItem"] {
        let Some(child) = part.get_child(name) else { return false; };
        part = child;
        draw.translate([part.x / 16.0, part.y / 16.0, part.z / 16.0]);
        draw.rotate_about([0.0, 0.0, 1.0], part.z_rot.to_degrees(), [0.0; 3]);
        draw.rotate_about([0.0, 1.0, 0.0], part.y_rot.to_degrees(), [0.0; 3]);
        draw.rotate_about([1.0, 0.0, 0.0], part.x_rot.to_degrees(), [0.0; 3]);
        draw.scale([part.x_scale, part.y_scale, part.z_scale]);
    }
    true
}
