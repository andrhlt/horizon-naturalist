//! Current Horizon renderers for the seven grassland/large animals.
//!
//! The meshes remain the Java-derived Naturalist models; this adapter keeps the
//! legacy geometry while using Horizon's current `EntityRenderer` surface.
use mod_api::{BlockEntityDrawContext, EntityRenderState, EntityRenderer, ModelPart, Pose};
use std::{collections::HashMap, sync::Mutex};

#[path = "../elephant_animations.rs"]
mod elephant_animations;
#[path = "../elephant_models.rs"]
mod elephant_models;
#[path = "../giraffe_animations.rs"]
mod giraffe_animations;
#[path = "../giraffe_models.rs"]
mod giraffe_models;
#[path = "../lion_animations.rs"]
mod lion_animations;
#[path = "../lion_models.rs"]
mod lion_models;
#[path = "../mammoth_models.rs"]
mod mammoth_models;
#[path = "../rhino_animations.rs"]
mod rhino_animations;
#[path = "../rhino_models.rs"]
mod rhino_models;
#[path = "../tiger_animations.rs"]
mod tiger_animations;
#[path = "../tiger_models.rs"]
mod tiger_models;
#[path = "../zebra_animations.rs"]
mod zebra_animations;
#[path = "../zebra_models.rs"]
mod zebra_models;

#[derive(Clone, Copy)]
enum Target {
    Position,
    Rotation,
    Scale,
}
enum Interpolation {
    Linear,
    CatmullRom,
    Mixed(&'static [usize]),
}
struct Channel {
    bone: &'static str,
    target: Target,
    interpolation: Interpolation,
    keys: &'static [(f32, [f32; 3])],
}
struct Clip {
    duration: f32,
    loops: bool,
    channels: &'static [Channel],
}

fn bone<'a>(part: &'a mut ModelPart, name: &str) -> Option<&'a mut ModelPart> {
    for (child_name, child) in part.children_mut() {
        if child_name == name {
            return Some(child);
        }
        if let Some(found) = bone(child, name) {
            return Some(found);
        }
    }
    None
}

impl Clip {
    fn pose(&self, model: &mut ModelPart, time: f32, weight: f32) {
        let time = if self.loops && self.duration > 0.0 {
            time.rem_euclid(self.duration)
        } else {
            time
        };
        for channel in self.channels {
            if channel.keys.is_empty() {
                continue;
            }
            let Some(part) = bone(model, channel.bone) else {
                continue;
            };
            let end = channel
                .keys
                .partition_point(|(key, _)| *key < time)
                .min(channel.keys.len() - 1);
            let first = end.saturating_sub(1);
            let (start_time, start) = channel.keys[first];
            let (end_time, end_value) = channel.keys[end];
            let t = if end_time == start_time {
                0.0
            } else {
                ((time - start_time) / (end_time - start_time)).clamp(0.0, 1.0)
            };
            let catmull = match channel.interpolation {
                Interpolation::Linear => false,
                Interpolation::CatmullRom => true,
                Interpolation::Mixed(keys) => keys.contains(&end),
            };
            let xyz = if catmull {
                let before = channel.keys[first.saturating_sub(1)].1;
                let after = channel.keys[(end + 1).min(channel.keys.len() - 1)].1;
                std::array::from_fn(|i| {
                    let [a, b, c, d] = [before[i], start[i], end_value[i], after[i]];
                    0.5 * (2.0 * b
                        + (-a + c) * t
                        + (2.0 * a - 5.0 * b + 4.0 * c - d) * t * t
                        + (-a + 3.0 * b - 3.0 * c + d) * t * t * t)
                })
            } else {
                std::array::from_fn(|i| start[i] + (end_value[i] - start[i]) * t)
            };
            match channel.target {
                Target::Position => {
                    part.offset_pos([xyz[0] * weight, -xyz[1] * weight, xyz[2] * weight])
                }
                Target::Rotation => {
                    part.offset_rotation(xyz.map(|angle| angle.to_radians() * weight))
                }
                Target::Scale => part.offset_scale(xyz.map(|scale| (scale - 1.0) * weight)),
            }
        }
    }
}

#[derive(Default)]
struct AnimationClock {
    previous: f32,
    factor: f32,
    phase_ms: i64,
    last_ms: Option<i64>,
}
impl AnimationClock {
    fn advance(&mut self, active: bool, tick: i32) {
        self.previous = self.factor;
        self.factor =
            (self.factor + ((if active { 1.0 } else { 0.0 }) - self.factor) * 0.5).clamp(0.0, 1.0);
        if active && self.last_ms.is_none() {
            self.last_ms = Some(i64::from(tick) * 50);
            self.phase_ms = 0;
        } else if !active && self.factor < 0.001 {
            self.last_ms = None;
        }
    }
    fn draw(
        &mut self,
        clip: Option<&Clip>,
        model: &mut ModelPart,
        age: f32,
        partial: f32,
        speed: f32,
        scale: f32,
    ) {
        let weight = (self.previous + partial * (self.factor - self.previous)) * scale;
        if weight <= 0.05 {
            return;
        }
        if let Some(last) = &mut self.last_ms {
            let now = (age * 50.0).floor() as i64;
            self.phase_ms += ((now - *last) as f32 * speed) as i64;
            *last = now;
        }
        if let Some(clip) = clip {
            clip.pose(model, self.phase_ms as f32 / 1000.0, weight);
        }
    }
}
#[derive(Default)]
struct EntityPose {
    tick: i32,
    clocks: [AnimationClock; 4],
    last_seen: i32,
}

#[derive(Clone, Copy)]
enum Kind {
    Elephant,
    Mammoth,
    Giraffe,
    Lion,
    Rhino,
    Tiger,
    Zebra,
}

pub(crate) struct LargeRenderer {
    kind: Kind,
    adult: ModelPart,
    baby: ModelPart,
    poses: Mutex<HashMap<u32, EntityPose>>,
}

impl LargeRenderer {
    fn new(kind: Kind, adult: ModelPart, baby: ModelPart) -> Self {
        Self {
            kind,
            adult,
            baby,
            poses: Mutex::new(HashMap::new()),
        }
    }
    pub(crate) fn elephant() -> Self {
        Self::new(
            Kind::Elephant,
            elephant_models::adult(),
            elephant_models::baby(),
        )
    }
    pub(crate) fn mammoth() -> Self {
        Self::new(
            Kind::Mammoth,
            mammoth_models::adult(),
            mammoth_models::baby(),
        )
    }
    pub(crate) fn giraffe() -> Self {
        Self::new(
            Kind::Giraffe,
            giraffe_models::adult(),
            giraffe_models::baby(),
        )
    }
    pub(crate) fn lion() -> Self {
        Self::new(Kind::Lion, lion_models::adult(), lion_models::baby())
    }
    pub(crate) fn rhino() -> Self {
        Self::new(Kind::Rhino, rhino_models::adult(), rhino_models::adult())
    }
    pub(crate) fn tiger() -> Self {
        Self::new(Kind::Tiger, tiger_models::adult(), tiger_models::baby())
    }
    pub(crate) fn zebra() -> Self {
        Self::new(Kind::Zebra, zebra_models::adult(), zebra_models::baby())
    }
    fn texture(&self, baby: bool) -> &'static str {
        match (self.kind, baby) {
            (Kind::Elephant, false) => "naturalist:entity/elephant/elephant",
            (Kind::Elephant, true) => "naturalist:entity/elephant/elephant_baby",
            (Kind::Mammoth, false) => "naturalist:entity/mammoth/mammoth",
            (Kind::Mammoth, true) => "naturalist:entity/mammoth/mammoth_baby",
            (Kind::Giraffe, false) => "naturalist:entity/giraffe/giraffe",
            (Kind::Giraffe, true) => "naturalist:entity/giraffe/giraffe_baby",
            (Kind::Lion, false) => "naturalist:entity/lion/lion",
            (Kind::Lion, true) => "naturalist:entity/lion/lion_baby",
            (Kind::Rhino, _) => "naturalist:entity/rhino",
            (Kind::Tiger, false) => "naturalist:entity/tiger/black_panther",
            (Kind::Tiger, true) => "naturalist:entity/tiger/black_panther_baby",
            (Kind::Zebra, false) => "naturalist:entity/zebra",
            (Kind::Zebra, true) => "naturalist:entity/zebra_baby",
        }
    }

    fn clips(&self, baby: bool) -> [Option<&'static Clip>; 4] {
        use self::{
            elephant_animations as elephant, giraffe_animations as giraffe,
            lion_animations as lion, rhino_animations as rhino, tiger_animations as tiger,
            zebra_animations as zebra,
        };
        match (self.kind, baby) {
            (Kind::Elephant | Kind::Mammoth, false) => [
                Some(&elephant::ADULT_IDLE),
                Some(&elephant::ADULT_WALK),
                Some(&elephant::ADULT_RUN),
                None,
            ],
            (Kind::Elephant | Kind::Mammoth, true) => [
                Some(&elephant::BABY_IDLE),
                Some(&elephant::BABY_WALK),
                Some(&elephant::BABY_RUN),
                None,
            ],
            (Kind::Giraffe, false) => [
                Some(&giraffe::ADULT_IDLE),
                Some(&giraffe::ADULT_WALK),
                Some(&giraffe::ADULT_RUN),
                None,
            ],
            (Kind::Giraffe, true) => [
                Some(&giraffe::BABY_IDLE),
                Some(&giraffe::BABY_WALK),
                Some(&giraffe::BABY_RUN),
                None,
            ],
            (Kind::Lion, false) => [
                Some(&lion::ADULT_IDLE),
                Some(&lion::ADULT_WALK),
                Some(&lion::ADULT_RUN),
                Some(&lion::ADULT_SLEEP),
            ],
            (Kind::Lion, true) => [
                Some(&lion::BABY_IDLE),
                Some(&lion::BABY_WALK),
                Some(&lion::BABY_RUN),
                Some(&lion::BABY_SLEEP),
            ],
            (Kind::Rhino, false) => [
                Some(&rhino::ADULT_IDLE),
                Some(&rhino::ADULT_WALK),
                Some(&rhino::ADULT_RUN),
                None,
            ],
            (Kind::Rhino, true) => [
                Some(&rhino::ADULT_IDLE),
                Some(&rhino::ADULT_WALK),
                Some(&rhino::ADULT_RUN),
                None,
            ],
            (Kind::Tiger, false) => [
                Some(&tiger::ADULT_IDLE),
                Some(&tiger::ADULT_WALK),
                Some(&tiger::ADULT_RUN),
                Some(&tiger::ADULT_SLEEP),
            ],
            (Kind::Tiger, true) => [
                Some(&tiger::BABY_IDLE),
                Some(&tiger::BABY_WALK),
                Some(&tiger::BABY_RUN),
                Some(&tiger::BABY_SLEEP),
            ],
            (Kind::Zebra, false) => [
                Some(&zebra::ADULT_IDLE),
                Some(&zebra::ADULT_WALK),
                Some(&zebra::ADULT_RUN),
                None,
            ],
            (Kind::Zebra, true) => [
                Some(&zebra::BABY_IDLE),
                Some(&zebra::BABY_WALK),
                Some(&zebra::BABY_RUN),
                None,
            ],
        }
    }
}

impl EntityRenderer for LargeRenderer {
    fn submit(
        &self,
        entity: &EntityRenderState,
        draw: &mut dyn BlockEntityDrawContext,
    ) -> Result<(), String> {
        let Some(living) = entity.living else {
            return Ok(());
        };
        let mut model = if living.is_baby {
            self.baby.clone()
        } else {
            self.adult.clone()
        };
        model.reset_all_poses();
        let tick = entity.age.floor() as i32;
        let moving = living.walk_animation_speed > 0.01;
        let running = moving && living.walk_animation_speed > 0.75;
        let sleeping = living.pose == Pose::Sleeping;
        let active = [
            !moving && !sleeping,
            moving && !running,
            moving && running,
            sleeping,
        ];
        let clips = self.clips(living.is_baby);
        let mut poses = self
            .poses
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if poses.get(&entity.id).is_some_and(|pose| tick < pose.tick) {
            poses.remove(&entity.id);
        }
        let pose = poses.entry(entity.id).or_default();
        if tick > pose.tick {
            for (clock, active) in pose.clocks.iter_mut().zip(active) {
                clock.advance(active, tick);
            }
            pose.tick = tick;
        }
        pose.last_seen = tick;
        let rates = [
            1.0,
            (living.walk_animation_speed / 0.35).clamp(0.4, 2.0),
            (living.walk_animation_speed / 0.75).clamp(0.4, 2.0),
            1.0,
        ];
        for i in 0..4 {
            pose.clocks[i].draw(
                clips[i],
                &mut model,
                entity.age,
                entity.partial,
                rates[i],
                1.0,
            );
        }
        if !sleeping {
            let head = match self.kind {
                Kind::Tiger => "skull",
                _ => "neck",
            };
            if let Some(part) = bone(&mut model, head) {
                part.x_rot += living.x_rot.to_radians();
                part.y_rot += living.y_rot.to_radians();
            }
        }
        if poses.len() > 2048 {
            poses.retain(|_, pose| tick.saturating_sub(pose.last_seen) < 1200);
        }
        drop(poses);
        draw.rotate_about([0.0, 1.0, 0.0], 180.0 - living.body_rot, [0.0; 3]);
        if living.death_time > 0.0 {
            let progress = (((living.death_time - 1.0) / 20.0 * 1.6).max(0.0))
                .sqrt()
                .min(1.0);
            draw.rotate_about([0.0, 0.0, 1.0], progress * 90.0, [0.0; 3]);
        } else if living.is_upside_down {
            draw.rotate_about([0.0, 0.0, 1.0], 180.0, [0.0; 3]);
        }
        draw.scale([living.scale, -living.scale, -living.scale]);
        draw.translate([0.0, -1.501 * living.age_scale, 0.0]);
        let tint = if living.has_red_overlay {
            0xFF7070
        } else {
            0xFFFFFF
        };
        if !draw.model_part(&model, self.texture(living.is_baby), tint) {
            return Err(format!(
                "missing Naturalist texture for {:?}",
                self.kind as u8
            ));
        }
        Ok(())
    }
}
