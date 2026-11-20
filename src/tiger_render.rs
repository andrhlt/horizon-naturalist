use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::{animal_rest_pose, appearance_texture},
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{Mob, goals::naturalist_cat_goals as tiger, model_layers::ModelPart};
use std::{
    cell::RefCell,
    collections::HashMap,
    time::{Duration, Instant},
};
use tiger_animations::*;

const STEP: &str = "naturalist:entity.tiger.step";
const SLEEP: &str = "naturalist:entity.tiger.sleep";
const PREY: &str = "naturalist:entity.tiger.prey";
const ATTACK: &str = "naturalist:entity.tiger.attack";
type Cue = (f32, &'static str, f32);
const TRACKS: [(f32, &[Cue]); 7] = [
    (0.0, &[(0.0, ATTACK, 1.0)]),
    (5.0, &[(0.0, SLEEP, 0.7)]),
    (2.0, &[(0.0, SLEEP, 0.7)]),
    (0.0, &[]),
    (
        1.0,
        &[
            (0.0, STEP, 0.4),
            (0.25, STEP, 0.4),
            (0.5, STEP, 0.4),
            (0.75, STEP, 0.4),
        ],
    ),
    (
        1.0,
        &[
            (0.0, PREY, 1.0),
            (0.2917, STEP, 0.25),
            (0.5, STEP, 0.25),
            (0.7917, STEP, 0.25),
            (0.9167, STEP, 0.25),
        ],
    ),
    (
        0.9167,
        &[
            (0.2083, STEP, 0.25),
            (0.25, STEP, 0.4),
            (0.7083, STEP, 0.25),
            (0.875, STEP, 0.4),
        ],
    ),
];

thread_local! {
    static MODELS: RefCell<[ModelPart; 2]> = RefCell::new([tiger_models::adult(), tiger_models::baby()]);
    static POSES: RefCell<HashMap<u32, TigerPose>> = RefCell::new(HashMap::new());
}

struct TigerPose {
    clocks: [AnimationClock; 7],
    last_seconds: [Option<f32>; 7],
    age: i32,
    attack_ticks: u8,
    last_seen: Instant,
}

impl TigerPose {
    fn new(now: Instant) -> Self {
        Self {
            clocks: Default::default(),
            last_seconds: Default::default(),
            age: -1,
            attack_ticks: 0,
            last_seen: now,
        }
    }

    fn update(&mut self, mob: &Mob, now: Instant, state: u8) -> bool {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::new(now);
        }
        self.last_seen = now;
        if age == self.age {
            return false;
        }
        self.age = age;
        if mob.ai.swinging && self.attack_ticks == 0 {
            self.attack_ticks = 5;
        } else {
            self.attack_ticks = self.attack_ticks.saturating_sub(1);
        }
        let sleep = state & 3;
        let stalking = state & 4 != 0;
        let running = state & 8 != 0;
        let moving = mob.velocity.x.powi(2) + mob.velocity.z.powi(2) > 1.0e-6;
        let active = [
            self.attack_ticks != 0,
            sleep == 1,
            sleep == 2,
            sleep == 0 && !moving,
            sleep == 0 && moving && !running && !stalking,
            sleep == 0 && moving && !running && stalking,
            sleep == 0 && moving && running,
        ];
        for (index, clock) in self.clocks.iter_mut().enumerate() {
            let smoothing = [1.0, 0.25, 0.25, 0.5, 0.5, 0.5, 0.5][index];
            clock.advance_with_rate(active[index], age, smoothing);
        }
        true
    }

    fn sounds(&mut self, mut play: impl FnMut(&'static str, f32)) {
        for (index, (length, cues)) in TRACKS.iter().enumerate() {
            let Some(now) = self.clocks[index]
                .started_phase_ms()
                .map(|ms| ms as f32 / 1000.0)
            else {
                self.last_seconds[index] = None;
                continue;
            };
            let previous = self.last_seconds[index]
                .replace(now)
                .filter(|&previous| previous <= now)
                .unwrap_or(-0.0001);
            for &(time, sound, volume) in *cues {
                let crossed = if *length > 0.0 {
                    ((now - time) / length).floor() > ((previous - time) / length).floor()
                } else {
                    previous < time && time <= now
                };
                if crossed {
                    play(sound, volume);
                }
            }
        }
    }
}

fn pose(poses: &mut HashMap<u32, TigerPose>, id: u32, now: Instant) -> &mut TigerPose {
    if !poses.contains_key(&id) {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(60));
    }
    poses.entry(id).or_insert_with(|| TigerPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_tiger_animations(&mut self) {
        if self.remote.is_none() {
            return;
        }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self
                .mobs
                .iter()
                .filter(|mob| !mob.removed && mob.kind.id == tiger::KIND)
            {
                let pose = pose(poses, mob.id, now);
                if pose.update(mob, now, animal_rest_pose(self, mob.id)) {
                    pose.sounds(|sound, volume| {
                        self.audio.play_unshared(
                            sound,
                            Some(mob.position.components()),
                            volume,
                            1.0,
                        );
                    });
                }
            }
        });
    }

    pub(super) fn draw_naturalist_tiger(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let texture = appearance_texture(self, mob.id, render.baby);
        let texture = texture.as_deref().unwrap_or(if render.baby {
            "naturalist:entity/tiger/black_panther_baby"
        } else {
            "naturalist:entity/tiger/black_panther"
        });
        let state = if self.remote.is_some() {
            animal_rest_pose(self, mob.id)
        } else {
            tiger::render_pose(mob, self.persist.uuids.get(&mob.id).copied())
        };
        let sleeping = state & 3 != 0;
        MODELS.with_borrow_mut(|models| {
            let model = &mut models[usize::from(render.baby)];
            model.reset_all_poses();
            for (name, visible) in if render.baby {
                [("awake", !sleeping), ("asleep", sleeping)]
            } else {
                [("sleep", sleeping), ("", true)]
            } {
                if !name.is_empty()
                    && let Some(part) = bone(model, name)
                {
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
                let clips: [Option<&Clip>; 7] = if render.baby {
                    [
                        None,
                        Some(&BABY_SLEEP),
                        Some(&BABY_SLEEP),
                        Some(&BABY_IDLE),
                        Some(&BABY_WALK),
                        Some(&BABY_WALK),
                        Some(&BABY_RUN),
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
                    ]
                };
                let walk = if render.baby { 2.5 } else { 2.0 };
                let prey = if render.baby { 2.5 } else { 0.8 };
                let speeds = [
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    motion * walk,
                    motion * prey,
                    motion * 1.5,
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
                if !sleeping
                    && let Some(head) = bone(model, if render.baby { "skull" } else { "skullRot" })
                {
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
