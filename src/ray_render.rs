use super::{Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_bass::BassPose, naturalist_deer_variants::appearance_texture,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone}};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};

struct RayPose {
    swim: BassPose,
    fin: [f32; 2],
    tail: [f32; 2],
    last_swim: Option<i64>,
    last_seen: Instant,
}

impl RayPose {
    fn new(now: Instant) -> Self {
        Self { swim: BassPose::new(now), fin: [0.0; 2], tail: [0.0; 2], last_swim: None, last_seen: now }
    }

    fn tick(&mut self, mob: &Mob, now: Instant) -> usize {
        let age = mob.age as i32;
        if age < self.swim.age { *self = Self::new(now); }
        self.last_seen = now;
        if age == self.swim.age { return 0; }
        let current = self.swim.clocks[1].started_phase_ms();
        let cues = current.map_or(0, |now| {
            let before = self.last_swim.filter(|old| *old <= now).unwrap_or(-1);
            let before = before as f32 / 1000.0;
            let now = now as f32 / 1000.0;
            [0.0, 1.0, 2.25, 3.5].into_iter().filter(|time| {
                ((now - time) / 6.0833).floor() > ((before - time) / 6.0833).floor()
            }).count()
        });
        self.swim.tick(mob, now);
        self.last_swim = current;
        let pitch = self.swim.pitch[1];
        let old_fin = self.fin[1];
        let old_tail = self.tail[1];
        self.fin = [old_fin, old_fin + (pitch - old_fin) * 0.15];
        self.tail = [old_tail, old_tail + (pitch - old_tail) * 0.09];
        cues
    }
}

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(ray_models::adult());
    static POSES: RefCell<HashMap<u32, RayPose>> = RefCell::new(HashMap::new());
}

fn pose(poses: &mut HashMap<u32, RayPose>, id: u32, now: Instant) -> &mut RayPose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| RayPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_ray_animations(&mut self) {
        if self.remote.is_none() { return; }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| !mob.removed && mob.kind.id == "naturalist:ray") {
                for _ in 0..pose(poses, mob.id, now).tick(mob, now) {
                    self.audio.play_unshared("naturalist:entity.ray.swim", Some(mob.position.components()), 0.3, 1.0);
                }
            }
        });
    }

    pub(super) fn draw_naturalist_ray(&self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw) -> bool {
        let texture = appearance_texture(self, mob.id, false);
        let texture = texture.as_deref().unwrap_or("naturalist:entity/ray/eagle_ray");
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() { pose.tick(mob, now); }
                use ray_animations::{ADULT_IDLE, ADULT_SWIM_IDLE_EVENT};
                pose.swim.clocks[0].draw(&ADULT_IDLE, model, render.age, render.partial, 1.0, 1.0);
                pose.swim.clocks[1].draw(&ADULT_SWIM_IDLE_EVENT, model, render.age, render.partial, 1.0, 1.0);
                let lerp = |[old, new]: [f32; 2]| old + (new - old) * render.partial;
                let pitch = lerp(pose.swim.pitch);
                if let Some(body) = bone(model, "body") { body.x_rot += pitch.to_radians(); }
                let fin = (pitch - lerp(pose.fin)).to_radians() * 1.6;
                for name in ["leftFin", "rightFin"] {
                    if let Some(part) = bone(model, name) { part.x_rot -= fin; }
                }
                if let Some(tail) = bone(model, "tail") {
                    tail.x_rot -= (pitch - lerp(pose.tail)).to_radians() * 1.6;
                }
                draw.rotate_about([0.0, 0.0, 1.0], -lerp(pose.swim.tilt), [0.0; 3]);
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(draw, model, texture, if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF })
        })
    }
}
