use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::appearance_texture,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_assets::item_model::DisplayContext;
use horizon_client::mobs::{Mob, goals::naturalist_vulture_goals::state, model_layers::ModelPart};
use std::{
    cell::RefCell,
    collections::HashMap,
    time::{Duration, Instant},
};

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(vulture_models::adult());
    static POSES: RefCell<HashMap<u32, VulturePose>> = RefCell::new(HashMap::new());
}

struct VulturePose {
    clocks: [AnimationClock; 2],
    age: i32,
    last_seen: Instant,
}

impl VulturePose {
    fn new(now: Instant) -> Self {
        Self {
            clocks: Default::default(),
            age: -1,
            last_seen: now,
        }
    }

    fn tick(&mut self, mob: &Mob, now: Instant) {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::new(now);
        }
        self.last_seen = now;
        if age == self.age {
            return;
        }
        self.age = age;
        let perched = state(mob).perched_ticks > 0;
        self.clocks[0].advance_with_rate(perched, age, 0.5);
        self.clocks[1].advance_with_rate(!perched, age, 0.5);
    }

    fn apply(&mut self, model: &mut ModelPart, render: &NativeMobRender<'_>) {
        self.clocks[0].draw(
            &vulture_animations::ADULT_SIT,
            model,
            render.age,
            render.partial,
            1.0,
            1.0,
        );
        self.clocks[1].draw(
            &vulture_animations::ADULT_FLY,
            model,
            render.age,
            render.partial,
            1.0,
            1.0,
        );
        apply_head_look(model, render);
    }
}

fn apply_head_look(model: &mut ModelPart, render: &NativeMobRender<'_>) {
    if let Some(neck) = bone(model, "neck") {
        neck.x_rot += render.pitch.to_radians();
        neck.y_rot += render.head_yaw.to_radians();
    }
}

fn pose(poses: &mut HashMap<u32, VulturePose>, id: u32, now: Instant) -> &mut VulturePose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| VulturePose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_vulture_animations(&self) {
        if self.remote.is_none() {
            return;
        }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self
                .mobs
                .iter()
                .filter(|mob| !mob.removed && mob.kind.id == "naturalist:vulture")
            {
                pose(poses, mob.id, now).tick(mob, now);
            }
        });
    }

    pub(super) fn draw_naturalist_vulture(
        &self,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let texture = appearance_texture(self, render.id, false);
        let texture = texture
            .as_deref()
            .unwrap_or("naturalist:entity/vulture/vulture");
        let mob = self.mobs.iter().find(|mob| mob.id == render.id);
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            if let Some(mob) = mob {
                POSES.with_borrow_mut(|poses| {
                    let now = Instant::now();
                    let pose = pose(poses, mob.id, now);
                    if self.remote.is_none() {
                        pose.tick(mob, now);
                    }
                    pose.apply(model, render);
                });
            }
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 {
                0xFF7070
            } else {
                0xFFFFFF
            };
            let drawn = self.native_draw_model_part(draw, model, texture, tint);
            if let Some(stack) = mob.and_then(|m| m.extra.main_hand_stack())
                && let Some(body) = model.get_child("body")
                && let Some(neck) = body.get_child("neck")
                && let Some(held) = neck.get_child("held_item")
            {
                draw.push();
                for part in [body, neck, held] {
                    let pose = part.store_pose();
                    draw.translate([pose.x / 16.0, pose.y / 16.0, pose.z / 16.0]);
                    draw.rotate_about([0.0, 0.0, 1.0], pose.z_rot.to_degrees(), [0.0; 3]);
                    draw.rotate_about([0.0, 1.0, 0.0], pose.y_rot.to_degrees(), [0.0; 3]);
                    draw.rotate_about([1.0, 0.0, 0.0], pose.x_rot.to_degrees(), [0.0; 3]);
                    draw.scale([pose.x_scale, pose.y_scale, pose.z_scale]);
                }
                draw.rotate_about([1.0, 0.0, 0.0], -90.0, [0.0; 3]);
                self.native_draw_item(draw, stack, DisplayContext::Ground, None, 0);
                draw.pop();
            }
            drawn
        })
    }
}
