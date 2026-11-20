use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::{appearance_texture, shark_aggressive},
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{
    Mob, living::wrap, model_layers::ModelPart, nav::naturalist_shark_state as shark,
};
use std::{cell::RefCell, collections::HashMap};

const SOUNDS: [&str; 4] = [
    "naturalist:entity.great_white_shark.attack",
    "naturalist:entity.great_white_shark.flop",
    "naturalist:entity.great_white_shark.swim",
    "naturalist:entity.great_white_shark.swim_fast",
];

#[derive(Default)]
struct SharkPose {
    motion: shark::Motion,
    clocks: [AnimationClock; 5],
    last_audio_phase: [Option<i64>; 4],
    tick: Option<i32>,
    was_swinging: bool,
    attack_ticks: i32,
}

impl SharkPose {
    fn tick(&mut self, mob: &Mob, aggressive: bool) -> [bool; 4] {
        let age = mob.age as i32;
        if self.tick.is_some_and(|tick| age < tick) {
            *self = Self::default();
        }
        if self.tick == Some(age) {
            return [false; 4];
        }
        let wet = mob.fluids.in_water;
        let moving = mob.velocity.horizontal_distance_squared() > 1.0e-6;
        let strike = mob.ai.swinging && !self.was_swinging;
        if strike {
            self.attack_ticks = 14;
        }
        for (index, (clock, enabled)) in self
            .clocks
            .iter_mut()
            .zip([
                self.attack_ticks > 0,
                !wet,
                wet && moving && !aggressive,
                wet && moving && aggressive,
                wet && !moving,
            ])
            .enumerate()
        {
            clock.advance_with_rate(enabled, age, if index == 0 { 1.0 } else { 0.5 });
        }
        self.motion.tick(mob.body_yaw, mob.velocity, wet);
        self.was_swinging = mob.ai.swinging;
        self.attack_ticks = (self.attack_ticks - 1).max(0);
        self.tick = Some(age);
        let mut cues = [strike, false, false, false];
        for (index, period) in [(1, 583), (2, 1500), (3, 917)] {
            let phase = self.clocks[index].started_phase_ms();
            cues[index] = phase.is_some_and(|current| {
                self.last_audio_phase[index].is_none_or(|previous| {
                    current < previous || current.div_euclid(period) > previous.div_euclid(period)
                })
            });
            self.last_audio_phase[index] = phase;
        }
        cues
    }
}

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(great_white_shark_models::adult());
    static POSES: RefCell<HashMap<u32, SharkPose>> = RefCell::new(HashMap::new());
}

impl Game<'_> {
    pub(in crate::game) fn shark_part_boxes(&self, mob: &Mob) -> [horizon_core::geometry::Aabb; 3] {
        let local = || {
            shark::motion(&mob.ai.nav).map_or((mob.body_yaw, 0.0), |state| (state.yaw, state.pitch))
        };
        let (yaw, pitch) = if self.remote.is_some() {
            POSES.with_borrow(|poses| {
                poses
                    .get(&mob.id)
                    .map_or_else(local, |state| (state.motion.yaw, state.motion.pitch))
            })
        } else {
            local()
        };
        shark::parts(mob.position, f64::from(mob.height()), yaw, pitch)
    }

    pub(super) fn tick_naturalist_shark_animations(&mut self) {
        if self.remote.is_none() {
            return;
        }
        POSES.with_borrow_mut(|poses| {
            if self.mobs.len() < poses.len() {
                poses.retain(|id, _| self.mobs.iter().any(|mob| mob.id == *id && !mob.removed));
            }
            for mob in self
                .mobs
                .iter()
                .filter(|mob| !mob.removed && mob.kind.id == shark::KIND)
            {
                let pose = poses.entry(mob.id).or_default();
                let cues = pose.tick(mob, shark_aggressive(self, mob));
                if mob.silent {
                    continue;
                }
                for sound in SOUNDS
                    .into_iter()
                    .zip(cues)
                    .filter_map(|(sound, play)| play.then_some(sound))
                {
                    self.audio
                        .play_unshared(sound, Some(mob.position.components()), 1.0, 1.0);
                }
            }
        });
    }

    pub(super) fn draw_naturalist_shark(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        use great_white_shark_animations::*;
        let texture = appearance_texture(self, render.id, render.baby);
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let pose = poses.entry(mob.id).or_default();
                pose.tick(mob, shark_aggressive(self, mob));
                let gait = (render.walk_speed / 0.5).clamp(0.4, 2.0);
                for (clock, (clip, speed)) in pose.clocks.iter_mut().zip([
                    (&ADULT_ATTACK, 1.0),
                    (&ADULT_FLOP, 1.0),
                    (&ADULT_SWIM, gait),
                    (&ADULT_SWIM_FAST, gait),
                    (&ADULT_IDLE, 1.0),
                ]) {
                    clock.draw(clip, model, render.age, render.partial, speed, 1.0);
                }
                let motion = &pose.motion;
                let lerp = |old: f32, new: f32| old + (new - old) * render.partial;
                let lerp_angle = |old: f32, new: f32| old + wrap(new - old) * render.partial;
                let yaw = lerp_angle(motion.previous_yaw, motion.yaw);
                let pitch = lerp(motion.previous_pitch, motion.pitch);
                let roll = lerp(motion.previous_roll, motion.roll);
                draw.rotate_about([0.0, 1.0, 0.0], wrap(render.body_yaw - yaw), [0.0; 3]);
                if let Some(root) = bone(model, "root") {
                    root.x_rot += pitch.to_radians();
                    root.z_rot += roll.to_radians();
                }
                for (index, (name, roll_factor)) in
                    [("skullRot", -0.35), ("tail", 0.0), ("fluke", 0.4)]
                        .into_iter()
                        .enumerate()
                {
                    if let Some(segment) = bone(model, name) {
                        let heading = lerp_angle(
                            motion.previous_segment_yaw[index],
                            motion.segment_yaw[index],
                        );
                        let vertical = lerp(
                            motion.previous_segment_pitch[index],
                            motion.segment_pitch[index],
                        );
                        let origin_yaw = if index <= 1 {
                            yaw
                        } else {
                            lerp_angle(
                                motion.previous_segment_yaw[index - 1],
                                motion.segment_yaw[index - 1],
                            )
                        };
                        let origin_pitch = if index <= 1 {
                            pitch
                        } else {
                            lerp(
                                motion.previous_segment_pitch[index - 1],
                                motion.segment_pitch[index - 1],
                            )
                        };
                        segment.y_rot += wrap(heading - origin_yaw).to_radians();
                        segment.x_rot += (vertical - origin_pitch).to_radians();
                        segment.z_rot += (roll * roll_factor).to_radians();
                    }
                }
            });
            let scale = mob.model_scale() * if render.baby { 0.5 } else { 1.0 };
            draw.scale([scale, -scale, -scale]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(
                draw,
                model,
                texture
                    .as_deref()
                    .unwrap_or("naturalist:entity/great_white_shark"),
                if render.hurt_time > 0 {
                    0xFF7070
                } else {
                    0xFFFFFF
                },
            )
        })
    }
}
