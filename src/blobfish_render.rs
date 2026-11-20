use super::{Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_bass::BassPose, naturalist_deer_variants::blobfish_appearance,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone}};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};

struct BlobfishPose {
    tilt: BassPose,
    clocks: [AnimationClock; 2],
    previous_swim_time: Option<i64>,
    last_seen: Instant,
}

impl BlobfishPose {
    fn new(now: Instant) -> Self {
        Self { tilt: BassPose::new(now), clocks: Default::default(), previous_swim_time: None, last_seen: now }
    }

    fn tick(&mut self, mob: &Mob, now: Instant) -> bool {
        let age = mob.age as i32;
        if age < self.tilt.age {
            *self = Self::new(now);
        }
        let mut cue = false;
        if age != self.tilt.age {
            let idle = mob.on_ground && mob.velocity.x.powi(2) + mob.velocity.z.powi(2) <= 1.0e-6;
            self.clocks[0].advance_with_rate(idle, age, 0.5);
            self.clocks[1].advance_with_rate(!idle, age, 0.5);
            let current = self.clocks[1].started_phase_ms();
            if let Some(now) = current {
                let last = self.previous_swim_time.filter(|old| *old <= now).unwrap_or(-1);
                cue = (now - 50).div_euclid(2000) > (last - 50).div_euclid(2000);
            }
            self.previous_swim_time = current;
        }
        self.tilt.tick(mob, now);
        self.last_seen = now;
        cue
    }
}

thread_local! {
    static MODELS: RefCell<[ModelPart; 2]> = RefCell::new([blobfish_models::pink(), blobfish_models::gray()]);
    static POSES: RefCell<HashMap<u32, BlobfishPose>> = RefCell::new(HashMap::new());
}

fn pose(poses: &mut HashMap<u32, BlobfishPose>, id: u32, now: Instant) -> &mut BlobfishPose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| BlobfishPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_blobfish_animations(&mut self) {
        if self.remote.is_none() { return; }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| !mob.removed && mob.kind.id == "naturalist:blobfish") {
                if pose(poses, mob.id, now).tick(mob, now) {
                    self.audio.play_unshared("naturalist:entity.blobfish.swim", Some(mob.position.components()), 0.3, 1.0);
                }
            }
        });
    }

    pub(super) fn draw_naturalist_blobfish(&self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw) -> bool {
        let (gray, texture) = blobfish_appearance(self, mob);
        let texture = texture.as_deref().unwrap_or(if gray {
            "naturalist:entity/blobfish/gray"
        } else {
            "naturalist:entity/blobfish/pink"
        });
        MODELS.with_borrow_mut(|models| {
            let model = &mut models[usize::from(gray)];
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.tick(mob, now);
                }
                use blobfish_animations::{ADULT_GRAY_SWIM, ADULT_PINK_IDLE};
                let gait = 2.0 * (render.walk_speed / 0.25).clamp(0.4, 2.0);
                pose.clocks[0].draw(&ADULT_PINK_IDLE, model, render.age, render.partial, 1.0, 1.0);
                pose.clocks[1].draw(&ADULT_GRAY_SWIM, model, render.age, render.partial, gait, 1.0);
                let lerp = |[old, new]: [f32; 2]| old + (new - old) * render.partial;
                if let Some(body) = bone(model, "body") {
                    body.x_rot += lerp(pose.tilt.pitch).to_radians();
                }
                draw.rotate_about([0.0, 0.0, 1.0], -lerp(pose.tilt.tilt), [0.0; 3]);
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(draw, model, texture, if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF })
        })
    }
}
