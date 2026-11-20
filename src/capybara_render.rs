use super::{
    Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_deer_variants::{appearance_texture, capybara_appearance},
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{Mob, model_layers::ModelPart, naturalist_capybara_sleep};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};
use capybara_animations::*;

thread_local! {
    static ADULT: RefCell<ModelPart> = RefCell::new(capybara_models::adult());
    static BABY: RefCell<ModelPart> = RefCell::new(capybara_models::baby());
    static POSES: RefCell<HashMap<u32, CapybaraPose>> = RefCell::new(HashMap::new());
}

struct CapybaraPose {
    clocks: [AnimationClock; 7],
    age: i32,
    last_seen: Instant,
}

impl CapybaraPose {
    fn new(now: Instant) -> Self {
        Self { clocks: Default::default(), age: -1, last_seen: now }
    }

    fn tick(&mut self, mob: &Mob, now: Instant, sitting: bool, sleep: u8) {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::new(now);
        }
        self.last_seen = now;
        if age == self.age {
            return;
        }
        self.age = age;
        let sleeping = sleep != 0;
        let sitting = !sleeping && sitting;
        let swimming = !sitting && !sleeping && mob.fluids.in_water;
        let moving = mob.velocity.x.powi(2) + mob.velocity.z.powi(2) > 1.0e-6;
        let walking = !sitting && !sleeping && !swimming && moving;
        for (clock, active) in self.clocks.iter_mut().zip([
            sitting, sleep == 1, sleep == 2, swimming, walking && !mob.extra.sprinting,
            walking && mob.extra.sprinting, !sitting && !sleeping && !swimming && !moving,
        ]) {
            clock.advance_with_rate(active, age, 0.5);
        }
    }
}

fn pose(poses: &mut HashMap<u32, CapybaraPose>, id: u32, now: Instant) -> &mut CapybaraPose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| CapybaraPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_capybara_animations(&self) {
        if self.remote.is_none() {
            return;
        }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| !mob.removed && mob.kind.id == "naturalist:capybara") {
                let (_, sitting, sleep) = capybara_appearance(self, mob.id);
                pose(poses, mob.id, now).tick(mob, now, sitting, sleep);
            }
        });
    }

    pub(super) fn draw_naturalist_capybara(
        &self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw,
    ) -> bool {
        let model = if render.baby { &BABY } else { &ADULT };
        let resolved = appearance_texture(self, mob.id, render.baby);
        let texture = resolved.as_deref().unwrap_or(if render.baby {
            "naturalist:entity/capybara/capybara_baby"
        } else {
            "naturalist:entity/capybara/capybara"
        });
        let (dye, sitting, sleep) = if self.remote.is_some() {
            capybara_appearance(self, mob.id)
        } else {
            (Some(mob.extra.color), mob.extra.sitting_pose, naturalist_capybara_sleep::pose(&mob.ai.nav))
        };
        model.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.tick(mob, now, sitting, sleep);
                }
                let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
                let land_speed = (render.walk_speed / gait).clamp(0.4, 2.0);
                let water_speed = (render.walk_speed / 0.35).clamp(0.4, 2.0);
                let (clips, rates) = if render.baby {
                    ([&BABY_SIT, &BABY_SLEEP, &BABY_SLEEP, &BABY_WALK, &BABY_WALK, &BABY_RUN, &BABY_IDLE],
                     [1.0, 1.0, 1.0, 4.0 * water_speed, 4.0 * land_speed, 3.0 * land_speed, 1.0])
                } else {
                    ([&ADULT_SIT, &ADULT_SLEEP, &ADULT_SLEEP2, &ADULT_SWIM, &ADULT_WALK, &ADULT_WALK, &BABY_IDLE],
                     [1.0, 1.0, 1.0, water_speed, 3.0 * land_speed, 4.0 * land_speed, 1.0])
                };
                for (index, (clip, rate)) in clips.into_iter().zip(rates).enumerate().take(if render.baby { 7 } else { 6 }) {
                    let weight = if index == 6 { 1.0 - (render.walk_speed * 2.5).min(1.0) } else { 1.0 };
                    pose.clocks[index].draw(clip, model, render.age, render.partial, rate, weight);
                }
                if sleep == 0 && !sitting && let Some(head) = bone(model, "skull") {
                    head.x_rot += render.pitch.to_radians();
                    head.y_rot += render.head_yaw.to_radians();
                }
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF };
            let drawn = self.native_draw_model_part(draw, model, texture, tint);
            if let Some(color) = dye.and_then(|id| horizon_client::mobs::render::DYE_NAMES.get(usize::from(id))) {
                let suffix = if render.baby && matches!(*color,
                    "white" | "magenta" | "light_blue" | "lime" | "gray" | "blue" | "brown" | "green" | "red" | "black"
                ) { "_baby" } else { "" };
                self.native_draw_model_part(draw, model, &format!("naturalist:entity/capybara/dye/{color}{suffix}"), tint);
            }
            drawn
        })
    }
}
