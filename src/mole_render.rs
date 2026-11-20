use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::{animal_rest_pose, appearance_texture},
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{
    Mob,
    goals::naturalist_mole_goals::{self as mole, Phase},
    model_layers::ModelPart,
};
use mole_animations::*;
use std::{
    cell::RefCell,
    collections::HashMap,
    time::{Duration, Instant},
};

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(mole_models::adult());
    static POSES: RefCell<HashMap<u32, Pose>> = RefCell::new(HashMap::new());
}

struct Pose {
    age: i32,
    phase: Phase,
    began: i32,
    last_seen: Instant,
    clocks: [AnimationClock; 7],
}

impl Pose {
    fn new(age: i32, phase: Phase, now: Instant) -> Self {
        Self {
            age: -1,
            phase,
            began: age,
            last_seen: now,
            clocks: Default::default(),
        }
    }
    fn step(&mut self, age: i32, phase: Phase, moving: bool, now: Instant) {
        if age < self.age {
            *self = Self::new(age, phase, now);
        }
        if self.phase != phase {
            self.phase = phase;
            self.began = age;
        }
        self.last_seen = now;
        if self.age == age {
            return;
        }
        self.age = age;
        let peeking = phase == Phase::Peeking;
        let surfaced = phase == Phase::Unrolled;
        for (clock, active) in self.clocks.iter_mut().zip([
            phase == Phase::Digging,
            phase == Phase::Underground,
            peeking && age - self.began < 13,
            peeking && age - self.began >= 13,
            phase == Phase::Unrolling,
            surfaced && !moving,
            surfaced && moving,
        ]) {
            clock.advance_with_rate(active, age, 0.25);
        }
    }
}

impl Game<'_> {
    pub(super) fn draw_naturalist_mole(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let phase = if self.remote.is_some() {
            Phase::from_saved(i32::from(animal_rest_pose(self, mob.id)))
        } else {
            mole::phase(mob)
        };
        let texture = appearance_texture(self, mob.id, false);
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            if let Some(mound) = bone(model, "mound") {
                mound.visible = phase != Phase::Unrolled;
            }
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                if !poses.contains_key(&mob.id) {
                    poses.retain(|_, pose| {
                        now.duration_since(pose.last_seen) < Duration::from_secs(60)
                    });
                }
                let age = mob.age as i32;
                let pose = poses
                    .entry(mob.id)
                    .or_insert_with(|| Pose::new(age, phase, now));
                pose.step(age, phase, render.walk_speed > 0.04, now);
                for (index, (clock, clip)) in pose
                    .clocks
                    .iter_mut()
                    .zip([
                        &ADULT_DIG_DOWN,
                        &ADULT_UNDERGROUND_HIDDEN,
                        &ADULT_SPAWN,
                        &ADULT_PEEK,
                        &ADULT_DIG_UP,
                        &ADULT_IDLE,
                        &ADULT_WALK,
                    ])
                    .enumerate()
                {
                    let speed = if index == 6 {
                        (render.walk_speed / 0.65 * 3.0).clamp(0.1, 5.0)
                    } else {
                        1.0
                    };
                    clock.draw(clip, model, render.age, render.partial, speed, 1.0);
                }
            });
            if phase == Phase::Unrolled
                && let Some(head) = bone(model, "skull")
            {
                head.x_rot += render.pitch.to_radians();
                head.y_rot += render.head_yaw.to_radians();
            }
            let scale = if render.baby { 0.5 } else { 1.0 };
            draw.scale([scale, -scale, -scale]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(
                draw,
                model,
                texture.as_deref().unwrap_or("naturalist:entity/mole"),
                if render.hurt_time > 0 {
                    0xFF7070
                } else {
                    0xFFFFFF
                },
            )
        })
    }
}
