use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::appearance_texture,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::{
    cell::RefCell,
    collections::HashMap,
    time::{Duration, Instant},
};

struct HippoPose {
    clocks: [AnimationClock; 7],
    bite_ticks: i32,
    tick: i32,
    last_seen: Instant,
}

thread_local! {
    static MODELS: RefCell<[ModelPart; 2]> = RefCell::new([hippo_models::adult(), hippo_models::baby()]);
    static POSES: RefCell<HashMap<u32, HippoPose>> = RefCell::new(HashMap::new());
}

impl Game<'_> {
    pub(super) fn draw_naturalist_hippo(
        &self,
        mob: &Mob,
        r: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        use hippo_animations::*;
        let texture = appearance_texture(self, r.id, r.baby);
        let (fallback, bite, pose, idle, swim_idle, walk, run, swim) = if r.baby {
            (
                "naturalist:entity/hippo/hippo_baby",
                &BABY_HEADBUTT,
                &BABY_SIT_IDLE,
                &BABY_IDLE,
                &BABY_IDLE,
                &BABY_WALK,
                &BABY_RUN,
                &BABY_WALK,
            )
        } else {
            (
                "naturalist:entity/hippo/hippo",
                &ADULT_BITE,
                &ADULT_SLEEP,
                &ADULT_IDLE,
                &ADULT_SWIM_IDLE,
                &ADULT_WALK,
                &ADULT_RUN,
                &ADULT_SWIM,
            )
        };
        MODELS.with_borrow_mut(|models| {
            let model = &mut models[usize::from(r.baby)];
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let tick = r.age.floor() as i32;
                if poses.len() > 1024 {
                    poses.retain(|_, p| now.duration_since(p.last_seen) < Duration::from_secs(30));
                }
                let new = || HippoPose {
                    clocks: Default::default(),
                    bite_ticks: 0,
                    tick: -1,
                    last_seen: now,
                };
                let current = poses.entry(r.id).or_insert_with(new);
                if tick < current.tick {
                    *current = new();
                }
                current.last_seen = now;
                if current.tick != tick {
                    if mob.ai.swinging && current.bite_ticks <= 0 {
                        current.bite_ticks = 20;
                    } else if current.bite_ticks > 0 {
                        current.bite_ticks -= 1;
                    }
                    let posing = super::naturalist_deer_variants::hippo_sitting(self, mob);
                    let wet = mob.fluids.in_water;
                    let moving = (mob.velocity.x * mob.velocity.x
                        + mob.velocity.z * mob.velocity.z)
                        > 1.0e-6;
                    let sprinting = mob.extra.sprinting;
                    for (index, (clock, active)) in current
                        .clocks
                        .iter_mut()
                        .zip([
                            current.bite_ticks > 0,
                            posing,
                            !posing && !moving && !wet,
                            !posing && !moving && wet,
                            !posing && moving && !wet && !sprinting,
                            !posing && moving && !wet && sprinting,
                            !posing && moving && wet,
                        ])
                        .enumerate()
                    {
                        clock.advance_with_rate(active, tick, if index == 0 { 1.0 } else { 0.5 });
                    }
                    current.tick = tick;
                }
                let idle_fade = 1.0 - (r.walk_speed * 2.5).min(1.0);
                let gait = (8.64 * mob.movement_speed()).min(1.0) as f32 * 0.65;
                let ground = (r.walk_speed / gait.max(0.05)).clamp(0.4, 2.0);
                let water = (r.walk_speed / 0.5).clamp(0.4, 2.0);
                for (clock, (clip, speed, weight)) in current.clocks.iter_mut().zip([
                    (bite, 1.0, 1.0),
                    (pose, 1.0, 1.0),
                    (idle, 0.8, idle_fade),
                    (swim_idle, 0.8, idle_fade),
                    (walk, 2.5 * ground, 1.0),
                    (run, 1.8 * ground, 1.0),
                    (swim, 1.1 * water, 1.0),
                ]) {
                    clock.draw(clip, model, r.age, r.partial, speed, weight);
                }
            });
            if let Some(neck) = bone(model, "neck") {
                neck.x_rot += r.pitch.to_radians();
                neck.y_rot += r.head_yaw.to_radians();
            }
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let drawn = self.native_draw_model_part(
                draw,
                model,
                texture.as_deref().unwrap_or(fallback),
                if r.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF },
            );
            if !r.baby
                && let Some(stack) = mob.extra.main_hand_stack()
            {
                draw.push();
                if jaw_pose(draw, model) {
                    draw.translate([-0.4, 0.76, -1.8]);
                    draw.scale([0.675; 3]);
                    self.native_draw_block(draw, &stack.item, &[]);
                }
                draw.pop();
            }
            drawn
        })
    }
}

fn jaw_pose(draw: &mut BlockEntityDraw, mut model: &ModelPart) -> bool {
    for name in ["root", "body", "bone", "neck", "botJaw"] {
        let Some(part) = model.get_child(name) else {
            return false;
        };
        model = part;
        draw.translate([part.x / 16.0, part.y / 16.0, part.z / 16.0]);
        draw.rotate_about([0.0, 0.0, 1.0], part.z_rot.to_degrees(), [0.0; 3]);
        draw.rotate_about([0.0, 1.0, 0.0], part.y_rot.to_degrees(), [0.0; 3]);
        draw.rotate_about([1.0, 0.0, 0.0], part.x_rot.to_degrees(), [0.0; 3]);
        draw.scale([part.x_scale, part.y_scale, part.z_scale]);
    }
    true
}
