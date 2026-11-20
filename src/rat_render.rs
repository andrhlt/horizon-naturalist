use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::{animal_rest_pose, appearance_texture},
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{Mob, goals::naturalist_rat_goals as rat, model_layers::ModelPart};
use rat_animations::*;
use std::{
    cell::RefCell,
    collections::HashMap,
    time::{Duration, Instant},
};

const STEP: &str = "naturalist:entity.rat.step";
const CLIPS: [&Clip; 7] = [
    &ADULT_SLEEP,
    &ADULT_SIT,
    &ADULT_STANDING,
    &ADULT_SWIM,
    &ADULT_IDLE,
    &ADULT_WALK,
    &ADULT_RUN,
];

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(rat_models::adult());
    static POSES: RefCell<HashMap<u32, RatPose>> = RefCell::new(HashMap::new());
}

struct RatPose {
    age: i32,
    seen: Instant,
    clocks: [AnimationClock; 7],
    last_steps: [Option<f32>; 2],
}
impl RatPose {
    fn new(seen: Instant) -> Self {
        Self {
            age: -1,
            seen,
            clocks: Default::default(),
            last_steps: Default::default(),
        }
    }
    fn update(&mut self, mob: &Mob, now: Instant, flags: u8) -> bool {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::new(now);
        }
        self.seen = now;
        if age == self.age {
            return false;
        }
        self.age = age;
        let sitting = flags & 1 != 0;
        let sleeping = !sitting && flags & 2 != 0;
        let swimming = !sitting && !sleeping && flags & 4 != 0;
        let free = !(sitting || sleeping || swimming);
        let interested = flags & 8 != 0;
        let speed = mob.velocity.x.powi(2) + mob.velocity.z.powi(2);
        let moving = speed > 1.0e-6;
        let running = moving && (flags & 16 != 0 || speed > 0.01);
        let active = [
            sleeping,
            sitting,
            free && !moving && interested,
            swimming,
            free && !moving && !interested,
            free && moving && !running,
            free && running,
        ];
        for (i, (clock, active)) in self.clocks.iter_mut().zip(active).enumerate() {
            clock.advance_with_rate(active, age, if i < 2 { 0.25 } else { 0.5 });
        }
        true
    }
    fn sounds(&mut self, mut play: impl FnMut(f32)) {
        const WALK: &[(f32, f32)] = &[(0.06, 0.25), (0.19, 0.15), (0.31, 0.25), (0.44, 0.15)];
        const RUN: &[(f32, f32)] = &[(0.42, 0.25), (0.58, 0.15)];
        for i in 0..2 {
            let (length, cues) = if i == 0 { (0.5, WALK) } else { (0.625, RUN) };
            let Some(now) = self.clocks[i + 5]
                .started_phase_ms()
                .map(|ms| ms as f32 / 1000.0)
            else {
                self.last_steps[i] = None;
                continue;
            };
            let previous = self.last_steps[i]
                .replace(now)
                .filter(|&before| before <= now)
                .unwrap_or(-0.0001);
            for &(time, volume) in cues {
                if ((now - time) / length).floor() > ((previous - time) / length).floor() {
                    play(volume);
                }
            }
        }
    }
}
fn pose(poses: &mut HashMap<u32, RatPose>, id: u32, now: Instant) -> &mut RatPose {
    if !poses.contains_key(&id) {
        poses.retain(|_, pose| now.duration_since(pose.seen) < Duration::from_secs(60));
    }
    poses.entry(id).or_insert_with(|| RatPose::new(now))
}
impl Game<'_> {
    pub(super) fn tick_naturalist_rat_animations(&mut self) {
        let now = Instant::now();
        let remote = self.remote.is_some();
        POSES.with_borrow_mut(|poses| {
            for mob in self
                .mobs
                .iter()
                .filter(|mob| mob.kind.id == rat::KIND && !mob.removed)
            {
                let state = if remote {
                    animal_rest_pose(self, mob.id)
                } else {
                    rat::render_pose(mob)
                };
                let pose = pose(poses, mob.id, now);
                if pose.update(mob, now, state) && !mob.silent {
                    pose.sounds(|volume| {
                        self.audio
                            .play_unshared(STEP, Some(mob.position.components()), volume, 1.0)
                    });
                }
            }
        });
    }
    pub(super) fn draw_naturalist_rat(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let texture = appearance_texture(self, mob.id, false);
        let texture = texture.as_deref().unwrap_or("naturalist:entity/rat/black");
        let flags = if self.remote.is_some() {
            animal_rest_pose(self, mob.id)
        } else {
            rat::render_pose(mob)
        };
        let sleeping = flags & 2 != 0;
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            for (name, visible) in [("awake", !sleeping), ("sleep", sleeping)] {
                if let Some(part) = bone(model, name) {
                    part.visible = visible;
                }
            }
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                pose.update(mob, now, flags);
                let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
                let motion = (render.walk_speed / gait).clamp(0.4, 2.0);
                let idle = 1.0 - (render.walk_speed * 2.5).clamp(0.0, 1.0);
                let speeds = [1.0, 1.0, 1.0, motion, 1.0, motion * 2.5, motion * 1.5];
                for (i, (clock, clip)) in pose.clocks.iter_mut().zip(CLIPS).enumerate() {
                    clock.draw(
                        clip,
                        model,
                        render.age,
                        render.partial,
                        speeds[i],
                        if i == 4 { idle } else { 1.0 },
                    );
                }
            });
            if flags & 3 == 0
                && let Some(head) = bone(model, "skull")
            {
                head.x_rot += render.pitch.to_radians();
                head.y_rot += render.head_yaw.to_radians();
            }
            let scale = if render.baby { 0.375 } else { 1.0 };
            draw.scale([scale, -scale, -scale]);
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
