use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::{animal_rest_pose, appearance_texture},
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{
    Mob,
    goals::{naturalist_cat_goals as cats, naturalist_lion_goals as lion},
    model_layers::ModelPart,
};
use lion_animations::*;
use std::{
    cell::RefCell,
    collections::HashMap,
    time::{Duration, Instant},
};

thread_local! {
    static MODELS: RefCell<[ModelPart; 2]> = RefCell::new([lion_models::adult(), lion_models::baby()]);
    static POSES: RefCell<HashMap<u32, LionPose>> = RefCell::new(HashMap::new());
}

struct LionPose {
    clocks: [AnimationClock; 8],
    age: i32,
    attack_ticks: u8,
    last_seen: Instant,
}

impl LionPose {
    fn new(now: Instant) -> Self {
        Self {
            clocks: Default::default(),
            age: -1,
            attack_ticks: 0,
            last_seen: now,
        }
    }

    fn update(&mut self, mob: &Mob, now: Instant, state: u8) {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::new(now);
        }
        self.last_seen = now;
        if age == self.age {
            return;
        }
        self.age = age;
        if mob.ai.swinging && self.attack_ticks == 0 {
            self.attack_ticks = 5;
        } else {
            self.attack_ticks = self.attack_ticks.saturating_sub(1);
        }
        let sleep = state & 3;
        let sitting = state & 16 != 0;
        let run = state & 8 != 0;
        let crouch = state & 4 != 0;
        let posing = sitting || sleep != 0;
        let moving = mob.velocity.x.powi(2) + mob.velocity.z.powi(2) > 1.0e-6;
        let active = [
            self.attack_ticks != 0,
            sleep == 1,
            sleep == 2,
            !posing && !moving,
            !posing && moving && !run && !crouch,
            !posing && moving && !run && crouch,
            !posing && moving && run,
            sitting,
        ];
        for (index, clock) in self.clocks.iter_mut().enumerate() {
            let smoothing = [1.0, 0.25, 0.25, 0.5, 0.5, 0.5, 0.5, 0.25][index];
            clock.advance_with_rate(active[index], age, smoothing);
        }
    }
}

fn pose(poses: &mut HashMap<u32, LionPose>, id: u32, now: Instant) -> &mut LionPose {
    if !poses.contains_key(&id) {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(60));
    }
    poses.entry(id).or_insert_with(|| LionPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_lion_animations(&self) {
        if self.remote.is_none() {
            return;
        }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self
                .mobs
                .iter()
                .filter(|mob| !mob.removed && mob.kind.id == lion::KIND)
            {
                pose(poses, mob.id, now).update(mob, now, animal_rest_pose(self, mob.id));
            }
        });
    }

    pub(super) fn draw_naturalist_lion(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let texture = appearance_texture(self, mob.id, render.baby);
        let texture = texture.as_deref().unwrap_or(if render.baby {
            "naturalist:entity/lion/lion_baby"
        } else {
            "naturalist:entity/lion/lion"
        });
        let state = if self.remote.is_some() {
            animal_rest_pose(self, mob.id)
        } else {
            cats::render_pose(mob, self.persist.uuids.get(&mob.id).copied())
        };
        let natural_sleep = state & 128 != 0;
        let angry = state & 64 != 0;
        let sleep_face = natural_sleep && !angry;
        MODELS.with_borrow_mut(|models| {
            let model = &mut models[usize::from(render.baby)];
            model.reset_all_poses();
            let visibility = if render.baby {
                [
                    ("awake", !sleep_face),
                    ("asleep", sleep_face),
                    ("angry", false),
                    ("mane", false),
                ]
            } else {
                [
                    ("awake", !sleep_face && !angry),
                    ("asleep", sleep_face),
                    ("angry", angry),
                    ("mane", state & 32 != 0),
                ]
            };
            for (name, visible) in visibility {
                if let Some(part) = bone(model, name) {
                    part.visible = visible;
                }
            }
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.update(mob, now, state);
                }
                let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
                let motion = (render.walk_speed / gait).clamp(0.4, 2.0);
                let idle = 1.0 - (render.walk_speed * 2.5).clamp(0.0, 1.0);
                let clips: [Option<&Clip>; 8] = if render.baby {
                    [
                        None,
                        Some(&BABY_SLEEP),
                        None,
                        Some(&BABY_IDLE),
                        Some(&BABY_WALK),
                        None,
                        Some(&BABY_RUN),
                        Some(&BABY_SIT_IDLE),
                    ]
                } else {
                    [
                        Some(&ADULT_ATTACK),
                        Some(&ADULT_SLEEP),
                        Some(&ADULT_SLEEP2),
                        Some(&ADULT_IDLE),
                        Some(&ADULT_WALK),
                        Some(&ADULT_PREY),
                        Some(&ADULT_RUN),
                        None,
                    ]
                };
                let speeds = [
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    motion * if render.baby { 1.0 } else { 2.0 },
                    motion * 0.8,
                    motion * 2.5,
                    1.0,
                ];
                for (index, (clock, clip)) in pose.clocks.iter_mut().zip(clips).enumerate() {
                    if let Some(clip) = clip {
                        clock.draw(
                            clip,
                            model,
                            render.age,
                            render.partial,
                            speeds[index],
                            if index == 3 { idle } else { 1.0 },
                        );
                    }
                }
                if !natural_sleep && let Some(head) = bone(model, "neck") {
                    head.x_rot += render.pitch.to_radians();
                    head.y_rot += render.head_yaw.to_radians();
                }
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(
                draw,
                model,
                texture,
                if render.hurt_time > 0 {
                    0xFF7070
                } else {
                    0xFFFFFF
                },
            )
        })
    }
}
