use super::{Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_deer_variants::{animal_rest_pose, appearance_texture},
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone}};
use horizon_client::mobs::{Mob, goals::naturalist_tortoise_goals as tortoise, model_layers::ModelPart};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};
use tortoise_animations::*;

thread_local! {
    static MODELS: RefCell<[ModelPart; 2]> = RefCell::new([tortoise_models::adult(), tortoise_models::baby()]);
    static POSES: RefCell<HashMap<u32, Pose>> = RefCell::new(HashMap::new());
}

struct Pose {
    clocks: [AnimationClock; 6],
    age: i32,
    last_seen: Instant,
}

impl Pose {
    fn new(now: Instant) -> Self { Self { clocks: Default::default(), age: -1, last_seen: now } }

    fn update(&mut self, mob: &Mob, now: Instant, flags: u8) {
        let age = mob.age as i32;
        if age < self.age { *self = Self::new(now); }
        self.last_seen = now;
        if age == self.age { return; }
        self.age = age;
        let hiding = flags & 1 != 0;
        let laying = flags & 2 != 0;
        let sitting = flags & 4 != 0;
        let moving = mob.velocity.x.powi(2) + mob.velocity.z.powi(2) > 1.0e-6;
        let states = [mob.hurt_time > 0, hiding, sitting, laying && !sitting,
            !sitting && !laying && !moving, !sitting && !laying && moving];
        for (clock, (active, rate)) in self.clocks.iter_mut().zip(states.into_iter().zip([1.0, 0.25, 0.25, 0.25, 0.5, 0.5])) {
            clock.advance_with_rate(active, age, rate);
        }
    }
}

fn pose(poses: &mut HashMap<u32, Pose>, id: u32, now: Instant) -> &mut Pose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| Pose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_tortoise_animations(&self) {
        if self.remote.is_none() { return; }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| mob.kind.id == tortoise::KIND && !mob.removed) {
                pose(poses, mob.id, now).update(mob, now, animal_rest_pose(self, mob.id));
            }
        });
    }

    pub(super) fn draw_naturalist_tortoise(&self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw) -> bool {
        let resolved = appearance_texture(self, mob.id, render.baby);
        let texture = resolved.as_deref().unwrap_or(if render.baby {
            "naturalist:entity/tortoise/brown_baby"
        } else { "naturalist:entity/tortoise/brown" });
        let mask = match mob.custom_name.as_deref() {
            Some("Donatello") => Some("naturalist:entity/tortoise/donatello"),
            Some("Leonardo") => Some("naturalist:entity/tortoise/leonardo"),
            Some("Michelangelo") => Some("naturalist:entity/tortoise/michelangelo"),
            Some("Raphael") => Some("naturalist:entity/tortoise/raphael"),
            _ => None,
        };
        MODELS.with_borrow_mut(|models| {
            let model = &mut models[usize::from(render.baby)];
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.update(mob, now, tortoise::render_pose(mob));
                }
                let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
                let land = (render.walk_speed / gait).clamp(0.4, 2.0);
                let idle = 1.0 - (render.walk_speed * 2.5).clamp(0.0, 1.0);
                let clips: [Option<&Clip>; 6] = if render.baby {
                    [None, None, Some(&BABY_SIT), None, Some(&BABY_IDLE), Some(&BABY_WALK)]
                } else { [Some(&ADULT_HURT), Some(&ADULT_HIDE), Some(&ADULT_SIT), Some(&ADULT_DIG), Some(&ADULT_IDLE), Some(&ADULT_WALK)] };
                let walk = if render.baby { 9.5 } else { 6.0 };
                for (i, (clock, clip)) in pose.clocks.iter_mut().zip(clips).enumerate() {
                    if let Some(clip) = clip {
                        clock.draw(clip, model, render.age, render.partial,
                            if i == 5 { walk * land } else { 1.0 }, if i == 4 { idle } else { 1.0 });
                    }
                }
                if let Some(neck) = bone(model, "neck") {
                    neck.x_rot += render.pitch.to_radians();
                    neck.y_rot += render.head_yaw.to_radians();
                }
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF };
            let drawn = self.native_draw_model_part(draw, model, texture, tint);
            if let Some(mask) = mask {
                self.native_draw_model_part(draw, model, mask, 0xFFFFFF);
            }
            drawn
        })
    }
}
