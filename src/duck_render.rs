use super::{Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone}};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};
use duck_animations::*;

struct DuckPose {
    clocks: [AnimationClock; 6],
    tick: i32,
    last_seen: Instant,
}

thread_local! {
    static ADULT: RefCell<ModelPart> = RefCell::new(duck_models::adult());
    static BABY: RefCell<ModelPart> = RefCell::new(duck_models::baby());
    static POSES: RefCell<HashMap<u32, DuckPose>> = RefCell::new(HashMap::new());
}

impl DuckPose {
    fn new(now: Instant) -> Self {
        Self { clocks: Default::default(), tick: -1, last_seen: now }
    }

    fn advance(&mut self, mob: &Mob, now: Instant, sitting: bool) {
        let tick = mob.age as i32;
        if tick < self.tick {
            *self = Self::new(now);
        }
        if tick > self.tick {
            let moving = mob.velocity.x * mob.velocity.x + mob.velocity.z * mob.velocity.z > 1.0e-6;
            let swimming = !sitting && mob.fluids.in_water;
            let walking = !sitting && !swimming && moving;
            let states = [sitting, !sitting && !swimming && !moving, swimming,
                walking && !mob.extra.sprinting, walking && mob.extra.sprinting,
                !mob.on_ground && !mob.fluids.in_water];
            for (index, (clock, active)) in self.clocks.iter_mut().zip(states).enumerate() {
                clock.advance_with_rate(active, tick, if index == 0 { 0.25 } else { 0.5 });
            }
            self.tick = tick;
        }
        self.last_seen = now;
    }
}

fn duck_pose(poses: &mut HashMap<u32, DuckPose>, id: u32, now: Instant) -> &mut DuckPose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| DuckPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_duck_animations(&self) {
        if self.remote.is_none() {
            return;
        }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| !mob.removed && mob.kind.id == "naturalist:duck") {
                let (_, _, sitting) = super::naturalist_deer_variants::duck_appearance(self, mob.id, mob.is_baby());
                duck_pose(poses, mob.id, now).advance(mob, now, sitting);
            }
        });
    }

    pub(super) fn draw_naturalist_duck(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let (model, texture) = if render.baby {
            (&BABY, "naturalist:entity/duck/duck_baby")
        } else if render.custom_name.is_some_and(|name| name.eq_ignore_ascii_case("queso")) {
            (&ADULT, "naturalist:entity/duck/queso")
        } else {
            (&ADULT, "naturalist:entity/duck/duck")
        };
        let (variant_texture, guest_dye, _) = super::naturalist_deer_variants::duck_appearance(self, mob.id, render.baby);
        let texture = variant_texture.as_deref().unwrap_or(texture);
        let dye = if self.remote.is_some() { guest_dye } else { Some(mob.extra.color) };
        model.with_borrow_mut(|part| {
            part.reset_all_poses();

            let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
            let gait_speed = (render.walk_speed / gait).clamp(0.4, 2.0);
            let swim_speed = (render.walk_speed / 0.5).clamp(0.4, 2.0);
            let clips = if render.baby {
                [&BABY_SIT_IDLE, &BABY_IDLE, &BABY_SWIM, &BABY_WALK, &BABY_WALK, &ADULT_FLAP]
            } else {
                [&ADULT_SIT, &ADULT_IDLE, &ADULT_SWIM, &ADULT_WALK, &ADULT_WALK, &ADULT_FLAP]
            };
            let speed = [1.0, 1.0, swim_speed, 1.5 * gait_speed, 2.0 * gait_speed, 1.0];
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = duck_pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.advance(mob, now, mob.extra.sitting_pose);
                }
                let weights = [1.0, 1.0 - (render.walk_speed * 2.5).min(1.0), 1.0, 1.0,
                    1.0, if render.baby { 0.0 } else { 1.0 }];
                for (index, clock) in pose.clocks.iter_mut().enumerate() {
                    clock.draw(clips[index], part, render.age, render.partial, speed[index], weights[index]);
                }
            });
            if let Some(neck) = bone(part, "neck") {
                neck.x_rot += render.pitch.to_radians();
                neck.y_rot += render.head_yaw.to_radians();
            }
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF };
            let drawn = self.native_draw_model_part(draw, part, texture, tint);
            if let Some(color) = dye.and_then(|id| horizon_client::mobs::render::DYE_NAMES.get(usize::from(id))) {
                let overlay = format!("naturalist:entity/duck/dye/{color}");
                self.native_draw_model_part(draw, part, &overlay, 0xFFFFFF);
            }
            drawn
        })
    }
}
