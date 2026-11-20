use super::{
    Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_deer_variants::bear_appearance,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{Mob, model_layers::ModelPart, naturalist_bears};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};

thread_local! {
    static BEAR: RefCell<ModelPart> = RefCell::new(bear_models::adult());
    static BEAR_CUB: RefCell<ModelPart> = RefCell::new(bear_models::baby());
    static BLACK_BEAR: RefCell<ModelPart> = RefCell::new(black_bear_models::adult());
    static BLACK_BEAR_CUB: RefCell<ModelPart> = RefCell::new(black_bear_models::baby());
    static POSES: RefCell<HashMap<u32, BearPose>> = RefCell::new(HashMap::new());
}

struct BearPose {
    clocks: [AnimationClock; 8],
    age: i32,
    last_seen: Instant,
}

impl BearPose {
    fn new(now: Instant) -> Self {
        Self { clocks: Default::default(), age: -1, last_seen: now }
    }

    fn tick(&mut self, mob: &Mob, now: Instant, pose: u8) {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::new(now);
        }
        self.last_seen = now;
        if age == self.age {
            return;
        }
        self.age = age;
        let moving = mob.velocity.x.powi(2) + mob.velocity.z.powi(2) > 1.0e-6;
        let active = pose & 15 == 0;
        for (clock, enabled) in self.clocks.iter_mut().zip([
            active,
            active && moving && !mob.extra.sprinting,
            active && moving && mob.extra.sprinting,
            pose & 1 != 0,
            pose & 4 != 0,
            pose & 2 != 0,
            pose & 8 != 0,
            mob.ai.swinging,
        ]) {
            clock.advance_with_rate(enabled, age, 0.5);
        }
    }
}

fn pose(poses: &mut HashMap<u32, BearPose>, id: u32, now: Instant) -> &mut BearPose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| BearPose::new(now))
}

fn show(model: &mut ModelPart, name: &str, visible: bool) {
    if let Some(part) = bone(model, name) {
        part.visible = visible;
    }
}

fn hide_cubes(model: &mut ModelPart, name: &str, hide: bool) {
    if let Some(part) = bone(model, name) {
        part.skip_draw = hide;
    }
}

fn set_details(model: &mut ModelPart, black: bool, baby: bool, pose: u8) {
    let sleeping = pose & 1 != 0;
    let angry = pose & 32 != 0;
    if baby {
        if !black {
            show(model, "awake", !sleeping);
            show(model, "asleep", sleeping);
        }
    } else if black {
        let berries = pose >> 6 == 1;
        let honey = pose >> 6 == 2;
        show(model, "sleep", sleeping);
        show(model, "head_angry", angry && !sleeping);
        show(model, "snout_angry", angry && !sleeping && !berries && !honey);
        hide_cubes(model, "snout", angry || berries || honey);
        for part in ["left_arm", "right_arm"] {
            hide_cubes(model, part, berries || honey);
        }
        for part in ["snout_berries", "left_arm_berries", "right_arm_berries"] {
            show(model, part, berries);
        }
        for part in ["snout_honey", "left_arm_honey", "right_arm_honey"] {
            show(model, part, honey);
        }
    } else {
        let sheared = pose & 16 != 0;
        show(model, "normalBody", !sheared);
        show(model, "shearedBody", sheared);
        show(model, "awake", !sleeping && !angry);
        show(model, "asleep", sleeping && !angry);
        show(model, "angry", angry);
        show(model, "angrySnout", angry);
        show(model, "berryArm", pose & 8 != 0 && pose >> 6 == 1);
        show(model, "honeyArm", pose & 8 != 0 && pose >> 6 == 2);
    }
}

macro_rules! adult_clips {
    ($module:ident) => {{
        use $module::*;
        [Some(&ADULT_IDLE), Some(&ADULT_WALK), Some(&ADULT_RUN), Some(&ADULT_SLEEP),
         Some(&ADULT_SIT), Some(&ADULT_SNIFF), Some(&ADULT_EAT), Some(&ADULT_ATTACK)]
    }};
}

macro_rules! baby_clips {
    ($module:ident) => {{
        use $module::*;
        [Some(&BABY_IDLE), Some(&BABY_WALK), Some(&BABY_RUN), Some(&BABY_SLEEP),
         Some(&BABY_SIT), Some(&BABY_SNIFF), None, None]
    }};
}

impl Game<'_> {
    pub(super) fn tick_naturalist_bear_animations(&self) {
        if self.remote.is_none() {
            return;
        }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| !mob.removed && naturalist_bears::is_bear(&mob.kind.id)) {
                let (_, bits) = bear_appearance(self, mob.id);
                pose(poses, mob.id, now).tick(mob, now, bits);
            }
        });
    }

    pub(super) fn draw_naturalist_bear(
        &self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw,
    ) -> bool {
        let black = mob.kind.id == "naturalist:black_bear";
        let baby = render.baby;
        let (model, texture, walk, run, head, clips) = match (black, baby) {
            (false, false) => (&BEAR, "naturalist:entity/bear/bear", 2.0, 1.0, "neck", adult_clips!(bear_animations)),
            (false, true) => (&BEAR_CUB, "naturalist:entity/bear/bear_baby", 2.5, 1.5, "neck", baby_clips!(bear_animations)),
            (true, false) => (&BLACK_BEAR, "naturalist:entity/black_bear/black_bear", 1.8, 2.0, "skullRot", adult_clips!(black_bear_animations)),
            (true, true) => (&BLACK_BEAR_CUB, "naturalist:entity/black_bear/black_bear_baby", 1.4, 2.0, "skull", baby_clips!(black_bear_animations)),
        };
        let (dye, bits) = if self.remote.is_some() {
            bear_appearance(self, mob.id)
        } else {
            ((mob.extra.color <= 15).then_some(mob.extra.color), naturalist_bears::render_pose(mob))
        };
        model.with_borrow_mut(|model| {
            model.reset_all_poses();
            set_details(model, black, baby, bits);
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.tick(mob, now, bits);
                }
                let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
                let speed = (render.walk_speed / gait).clamp(0.4, 2.0);
                let rates = [1.0, walk * speed, run * speed, 1.0, 1.0, 1.0, 1.0, 1.3];
                for (index, (clip, rate)) in clips.into_iter().zip(rates).enumerate() {
                    if let Some(clip) = clip {
                        let weight = if index == 0 { 1.0 - (render.walk_speed * 2.5).min(1.0) } else { 1.0 };
                        pose.clocks[index].draw(clip, model, render.age, render.partial, rate, weight);
                    }
                }
                if bits & 13 == 0 && let Some(head) = bone(model, head) {
                    head.x_rot += render.pitch.to_radians();
                    head.y_rot += render.head_yaw.to_radians();
                }
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF };
            let drawn = self.native_draw_model_part(draw, model, texture, tint);
            if let Some(color) = dye.and_then(|id| horizon_client::mobs::render::DYE_NAMES.get(usize::from(id))) {
                let species = if black { "black_bear" } else { "bear" };
                self.native_draw_model_part(draw, model, &format!("naturalist:entity/{species}/dye/{color}"), tint);
            }
            drawn
        })
    }
}
