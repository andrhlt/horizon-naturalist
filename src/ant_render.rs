use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::appearance_texture,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target},
};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::{
    cell::RefCell,
    collections::HashMap,
    time::{Duration, Instant},
};

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(ant_models::adult());
    static POSES: RefCell<HashMap<u32, AntPose>> = RefCell::new(HashMap::new());
}

struct AntPose {
    idle: AnimationClock,
    walk: AnimationClock,
    tick: i32,
    last_seen: Instant,
}

impl AntPose {
    fn new(now: Instant) -> Self {
        Self {
            idle: Default::default(),
            walk: Default::default(),
            tick: -1,
            last_seen: now,
        }
    }

    fn advance(&mut self, mob: &Mob, now: Instant) {
        let tick = mob.age as i32;
        if tick < self.tick {
            *self = Self::new(now);
        }
        self.last_seen = now;
        if tick == self.tick {
            return;
        }
        self.tick = tick;
        let moving = mob.velocity.x * mob.velocity.x + mob.velocity.z * mob.velocity.z > 1e-6;
        self.idle.advance_with_rate(!moving, tick, 0.5);
        self.walk.advance_with_rate(moving, tick, 0.5);
    }

    fn apply(&mut self, mob: &Mob, render: &NativeMobRender<'_>, model: &mut ModelPart) {
        let idle_fade = 1.0 - (render.walk_speed * 2.5).min(1.0);
        let gait = ((8.64 * mob.attrs.speed).min(1.0) * 0.65) as f32;
        let walk = if mob.climbing {
            1.0
        } else {
            3.0 * (render.walk_speed / gait.max(0.05)).clamp(0.4, 2.0)
        };
        self.idle.draw(
            &ant_animations::ADULT_IDLE,
            model,
            render.age,
            render.partial,
            1.0,
            idle_fade,
        );
        self.walk.draw(
            &ant_animations::ADULT_WALK,
            model,
            render.age,
            render.partial,
            walk,
            1.0,
        );
    }
}

fn pose(poses: &mut HashMap<u32, AntPose>, id: u32, now: Instant) -> &mut AntPose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| AntPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_ant_animations(&self) {
        if self.remote.is_none() {
            return;
        }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self
                .mobs
                .iter()
                .filter(|mob| !mob.removed && mob.kind.id == "naturalist:ant")
            {
                pose(poses, mob.id, now).advance(mob, now);
            }
        });
    }

    pub(super) fn draw_naturalist_ant(
        &self,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let texture = appearance_texture(self, render.id, false);
        let texture = texture.as_deref().unwrap_or("naturalist:entity/ant");
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            if let Some(mob) = self.mobs.iter().find(|mob| mob.id == render.id) {
                POSES.with_borrow_mut(|poses| {
                    let now = Instant::now();
                    let pose = pose(poses, mob.id, now);
                    if self.remote.is_none() {
                        pose.advance(mob, now);
                    }
                    pose.apply(mob, render, model);
                });
            }
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 {
                0xFF7070
            } else {
                0xFFFFFF
            };
            self.native_draw_model_part(draw, model, texture, tint)
        })
    }
}
