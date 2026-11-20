use super::{Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_deer_variants::appearance_texture,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone}};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use horizon_core::geometry::Vec3;
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};

struct JellyfishPose {
    clocks: [AnimationClock; 3],
    age: i32,
    pitch: [f32; 2],
    last_move: Vec3,
    previous_swim_time: Option<i64>,
    last_seen: Instant,
}

impl JellyfishPose {
    fn new(now: Instant) -> Self {
        Self { clocks: Default::default(), age: -1, pitch: [0.0; 2], last_move: Vec3::ZERO,
            previous_swim_time: None, last_seen: now }
    }

    fn tick(&mut self, mob: &Mob, now: Instant) -> bool {
        let age = mob.age as i32;
        if age < self.age { *self = Self::new(now); }
        self.last_seen = now;
        if age == self.age { return false; }
        self.age = age;
        let water = mob.fluids.in_water;
        let speed_squared = mob.velocity.length_squared();
        let moving = speed_squared > 1.0e-5;
        self.clocks[0].advance_with_rate(!water, age, 0.5);
        self.clocks[1].advance_with_rate(water && !moving, age, 0.5);
        self.clocks[2].advance_with_rate(water && moving, age, 0.5);
        let mut target = 0.0;
        if water {
            if speed_squared > 1.0e-6 { self.last_move = mob.velocity; }
            let move_dir = self.last_move;
            target = -(move_dir.x.hypot(move_dir.z).atan2(move_dir.y).to_degrees() as f32);
        }
        let old = self.pitch[1];
        self.pitch = [old, old + (target - old) * 0.1];
        let phase = self.clocks[2].started_phase_ms();
        let cue = phase.is_some_and(|phase| phase.div_euclid(1000)
            > self.previous_swim_time.filter(|old| *old <= phase).unwrap_or(-1).div_euclid(1000));
        self.previous_swim_time = phase;
        cue
    }
}

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(jellyfish_models::adult());
    static POSES: RefCell<HashMap<u32, JellyfishPose>> = RefCell::new(HashMap::new());
}

fn pose(poses: &mut HashMap<u32, JellyfishPose>, id: u32, now: Instant) -> &mut JellyfishPose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| JellyfishPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_jellyfish_animations(&mut self) {
        if self.remote.is_none() { return; }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| !mob.removed && mob.kind.id == "naturalist:jellyfish") {
                if pose(poses, mob.id, now).tick(mob, now) {
                    self.audio.play_unshared("naturalist:entity.jellyfish.swim", Some(mob.position.components()), 0.4, 1.0);
                }
            }
        });
    }

    pub(super) fn draw_naturalist_jellyfish(&self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw) -> bool {
        let texture = appearance_texture(self, mob.id, false);
        let texture = texture.as_deref().unwrap_or("naturalist:entity/jellyfish/white");
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() { pose.tick(mob, now); }
                use jellyfish_animations::{ADULT_IDLE, ADULT_LAND, ADULT_SWIM};
                for (clock, clip) in pose.clocks.iter_mut().zip([&ADULT_LAND, &ADULT_IDLE, &ADULT_SWIM]) {
                    clock.draw(clip, model, render.age, render.partial, 1.0, 1.0);
                }
                let [old, new] = pose.pitch;
                if let Some(body) = bone(model, "body") {
                    body.x_rot -= (old + (new - old) * render.partial).to_radians();
                }
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(draw, model, texture, if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF })
        })
    }
}
