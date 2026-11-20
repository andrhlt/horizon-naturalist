use super::{Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_bass::BassPose, naturalist_deer_variants::appearance_texture,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone}};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};

struct PiranhaPose {
    swim: BassPose,
    attack: AnimationClock,
    attack_ticks: u8,
    last_seen: Instant,
}

impl PiranhaPose {
    fn new(now: Instant) -> Self {
        Self { swim: BassPose::new(now), attack: AnimationClock::default(), attack_ticks: 0, last_seen: now }
    }

    fn tick(&mut self, mob: &Mob, now: Instant) {
        let age = mob.age as i32;
        if age < self.swim.age {
            *self = Self::new(now);
        }
        if age != self.swim.age {
            if mob.ai.swinging && self.attack_ticks == 0 {
                self.attack_ticks = 15;
            } else {
                self.attack_ticks = self.attack_ticks.saturating_sub(1);
            }
            self.attack.advance_with_rate(self.attack_ticks > 0, age, 1.0);
        }
        self.swim.tick(mob, now);
        self.last_seen = now;
    }
}

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(piranha_models::adult());
    static POSES: RefCell<HashMap<u32, PiranhaPose>> = RefCell::new(HashMap::new());
}

fn pose(poses: &mut HashMap<u32, PiranhaPose>, id: u32, now: Instant) -> &mut PiranhaPose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| PiranhaPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_piranha_animations(&self) {
        if self.remote.is_none() { return; }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| !mob.removed && mob.kind.id == "naturalist:piranha") {
                pose(poses, mob.id, now).tick(mob, now);
            }
        });
    }

    pub(super) fn draw_naturalist_piranha(&self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw) -> bool {
        let texture = appearance_texture(self, mob.id, false);
        let texture = texture.as_deref().unwrap_or("naturalist:entity/piranha");
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.tick(mob, now);
                }
                use piranha_animations::{ADULT_ATTACK, ADULT_FLOP, ADULT_SWIM};
                let gait = 2.0 * (render.walk_speed / 0.25).clamp(0.4, 2.0);
                pose.attack.draw(&ADULT_ATTACK, model, render.age, render.partial, 1.0, 1.0);
                pose.swim.clocks[0].draw(&ADULT_FLOP, model, render.age, render.partial, 1.0, 1.0);
                pose.swim.clocks[1].draw(&ADULT_SWIM, model, render.age, render.partial, gait, 1.0);
                let lerp = |[old, new]: [f32; 2]| old + (new - old) * render.partial;
                if let Some(body) = bone(model, "root") {
                    body.x_rot += lerp(pose.swim.pitch).to_radians();
                }
                draw.rotate_about([0.0, 0.0, 1.0], -lerp(pose.swim.tilt), [0.0; 3]);
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(draw, model, texture, if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF })
        })
    }
}
