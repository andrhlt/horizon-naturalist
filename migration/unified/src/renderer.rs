//! Naturalist 2.0.7 deer models and keyframes, adapted to Horizon's renderer API.
use super::appearance::DeerAppearance;
use mod_api::{BlockEntityDrawContext, EntityRenderState, EntityRenderer, ModelPart};
use std::{collections::HashMap, sync::Mutex};

#[path = "animations.rs"]
mod animations;
#[path = "models.rs"]
#[allow(unused_variables, non_snake_case)]
mod models;

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
            let blend = if end_time == start_time {
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
                        + (-a + c) * blend
                        + (2.0 * a - 5.0 * b + 4.0 * c - d) * blend * blend
                        + (-a + 3.0 * b - 3.0 * c + d) * blend * blend * blend)
                })
            } else {
                std::array::from_fn(|i| start[i] + (end_value[i] - start[i]) * blend)
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
    previous_factor: f32,
    factor: f32,
    phase_ms: i64,
    last_ms: Option<i64>,
}
impl AnimationClock {
    fn advance(&mut self, active: bool, tick: i32) {
        self.previous_factor = self.factor;
        let target = if active { 1.0 } else { 0.0 };
        self.factor = (self.factor + (target - self.factor) * 0.5).clamp(0.0, 1.0);
        if active && self.last_ms.is_none() {
            self.last_ms = Some(i64::from(tick) * 50);
            self.phase_ms = 0;
        } else if !active && self.factor < 0.001 {
            self.last_ms = None;
        }
    }
    fn draw(
        &mut self,
        clip: &Clip,
        model: &mut ModelPart,
        age: f32,
        partial: f32,
        speed: f32,
        scale: f32,
    ) {
        let weight =
            (self.previous_factor + partial * (self.factor - self.previous_factor)) * scale;
        if weight <= 0.05 {
            return;
        }
        if let Some(last) = &mut self.last_ms {
            let now = (age * 50.0).floor() as i64;
            self.phase_ms += ((now - *last) as f32 * speed) as i64;
            *last = now;
        }
        clip.pose(model, self.phase_ms as f32 / 1000.0, weight);
    }
}
struct DeerPose {
    clocks: [AnimationClock; 4],
    tick: i32,
    handle: mod_api::EntityRef,
    height: Option<f32>,
    appearance: Result<Option<DeerAppearance>, String>,
}
impl DeerPose {
    fn new(handle: mod_api::EntityRef) -> Self {
        Self {
            clocks: Default::default(),
            tick: -1,
            handle,
            height: None,
            appearance: Ok(None),
        }
    }
}
pub(crate) struct DeerRenderer {
    adult: ModelPart,
    baby: ModelPart,
    poses: Mutex<HashMap<u32, DeerPose>>,
}
impl DeerRenderer {
    pub(crate) fn new() -> Self {
        Self {
            adult: models::adult(),
            baby: models::baby(),
            poses: Mutex::new(HashMap::new()),
        }
    }
    pub(crate) fn observe(
        &self,
        handle: mod_api::EntityRef,
        tick: i32,
        moving: bool,
        height: Option<f32>,
        appearance: Result<Option<DeerAppearance>, String>,
    ) {
        let mut poses = self
            .poses
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let pose = poses
            .entry(handle.id())
            .or_insert_with(|| DeerPose::new(handle));
        if pose.handle != handle || tick < pose.tick {
            *pose = DeerPose::new(handle);
        }
        if tick > pose.tick {
            let eating = appearance
                .as_ref()
                .ok()
                .and_then(Option::as_ref)
                .is_some_and(|a| a.eating);
            let sprinting = appearance
                .as_ref()
                .ok()
                .and_then(Option::as_ref)
                .is_some_and(|a| a.sprinting);
            for (clock, active) in pose.clocks.iter_mut().zip([
                eating,
                !moving,
                moving && !sprinting,
                moving && sprinting,
            ]) {
                clock.advance(active, tick);
            }
            pose.tick = tick;
        }
        pose.height = height;
        pose.appearance = appearance;
    }
    pub(crate) fn retain_live(&self, host: &dyn mod_api::ClientHost) {
        let mut poses = self
            .poses
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        poses.retain(|&id, pose| host.entity(id) == Some(pose.handle));
    }
    pub(crate) fn clear(&self) {
        self.poses
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear();
    }
}
impl EntityRenderer for DeerRenderer {
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
        for name in ["sleep", "saddle"] {
            if let Some(part) = bone(&mut model, name) {
                part.visible = false;
            }
        }
        let walk_speed = living.walk_animation_speed;
        let mut poses = self
            .poses
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let appearance = match poses.get(&entity.id) {
            Some(pose) => pose
                .appearance
                .clone()
                .map_err(|error| format!("deer appearance: {error}"))?,
            None => None,
        };
        let movement_speed = appearance.as_ref().map_or(0.2, |a| a.movement_speed);
        let gait = ((8.64 * movement_speed).min(1.0) * 0.65).max(0.05);
        let speed = (walk_speed / gait).clamp(0.4, 2.0);
        let (clips, rates) = if living.is_baby {
            (
                [
                    &animations::BABY_EAT,
                    &animations::BABY_IDLE,
                    &animations::BABY_WALK,
                    &animations::BABY_RUN,
                ],
                [1.0, 1.5, 1.7 * speed, speed],
            )
        } else {
            (
                [
                    &animations::ADULT_EAT,
                    &animations::ADULT_IDLE,
                    &animations::ADULT_WALK,
                    &animations::ADULT_RUN,
                ],
                [1.0, 1.0, 2.4 * speed, 2.3 * speed],
            )
        };
        let height = poses.get(&entity.id).and_then(|pose| pose.height);
        if let Some(pose) = poses.get_mut(&entity.id) {
            let weights = [1.0, 1.0 - (walk_speed * 2.5).min(1.0), 1.0, 1.0];
            for i in 0..4 {
                pose.clocks[i].draw(
                    clips[i],
                    &mut model,
                    entity.age,
                    entity.partial,
                    rates[i],
                    weights[i],
                );
            }
        }
        drop(poses);
        if !appearance.as_ref().is_some_and(|a| a.eating)
            && let Some(neck) = bone(&mut model, "neck")
        {
            neck.x_rot += living.x_rot.to_radians();
            neck.y_rot += living.y_rot.to_radians();
        }
        draw.scale([living.scale; 3]);
        draw.rotate_about([0.0, 1.0, 0.0], 180.0 - living.body_rot, [0.0; 3]);
        if living.death_time > 0.0 {
            let progress = (((living.death_time - 1.0) / 20.0 * 1.6).max(0.0))
                .sqrt()
                .min(1.0);
            draw.rotate_about([0.0, 0.0, 1.0], progress * 90.0, [0.0; 3]);
        } else if living.is_upside_down {
            if let Some(height) = height {
                draw.translate([0.0, height + 0.1, 0.0]);
                draw.rotate_about([0.0, 0.0, 1.0], 180.0, [0.0; 3]);
            }
        }
        draw.scale([1.0, -1.0, -1.0]);
        draw.translate([0.0, -1.501, 0.0]);
        let fallback = DeerAppearance::default();
        let appearance = appearance.as_ref().unwrap_or(&fallback);
        let texture = if living.is_baby {
            &appearance.baby_texture
        } else {
            &appearance.adult_texture
        };
        if !draw.model_part(
            &model,
            texture,
            if living.has_red_overlay {
                0xFF7070
            } else {
                0xFFFFFF
            },
        ) {
            return Err(format!("missing deer texture {texture}"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adult_and_fawn_preserve_source_pivots_and_geometry() {
        let mut adult = models::adult();
        let mut baby = models::baby();
        let body = bone(&mut adult, "body").unwrap();
        assert_eq!([body.x, body.y, body.z], [0.0, -16.0, 0.0]);
        assert_eq!(body.polygons.len(), 6);
        assert_eq!(bone(&mut adult, "antlers").unwrap().polygons.len(), 24);
        let body = bone(&mut baby, "body").unwrap();
        assert_eq!([body.x, body.y, body.z], [0.0, -8.5, 0.0]);
        assert_eq!(body.polygons.len(), 6);
        assert!(bone(&mut baby, "antlers").is_none());
    }

    #[test]
    fn source_walk_clip_interpolates_body_rotation() {
        let mut model = models::adult();
        animations::ADULT_WALK.pose(&mut model, 0.125, 1.0);
        let body = bone(&mut model, "body").unwrap();
        assert!((body.x_rot.to_degrees() - 1.0).abs() < 1.0e-5);
        assert!((body.z_rot.to_degrees() - 0.7).abs() < 1.0e-5);
    }

    #[test]
    fn clock_blends_on_ticks_and_keeps_phase() {
        let mut clock = AnimationClock::default();
        clock.advance(true, 10);
        assert_eq!(clock.factor, 0.5);
        let mut model = models::adult();
        clock.draw(&animations::ADULT_IDLE, &mut model, 10.5, 0.5, 1.0, 1.0);
        assert_eq!(clock.phase_ms, 25);
        clock.advance(false, 11);
        assert_eq!(clock.factor, 0.25);
    }
    #[derive(Default)]
    struct Draw {
        sprite: String,
        tint: u32,
        has_antlers: bool,
    }
    impl BlockEntityDrawContext for Draw {
        fn rotate_about(&mut self, _: [f32; 3], _: f32, _: [f32; 3]) {}
        fn scale(&mut self, _: [f32; 3]) {}
        fn translate(&mut self, _: [f32; 3]) {}
        fn model_part(&mut self, part: &ModelPart, sprite: &str, tint: u32) -> bool {
            self.sprite = sprite.into();
            self.tint = tint;
            self.has_antlers = bone(&mut part.clone(), "antlers").is_some();
            true
        }
    }
    #[test]
    fn missing_state_uses_default_adult_and_fawn_textures() {
        let renderer = DeerRenderer::new();
        for baby in [false, true] {
            let living: mod_api::LivingRenderState = mod_api::__private::built::LivingRenderState {
                body_rot: 0.0,
                y_rot: 0.0,
                x_rot: 0.0,
                death_time: 0.0,
                walk_animation_pos: 0.0,
                walk_animation_speed: 0.0,
                scale: 1.0,
                age_scale: if baby { 0.5 } else { 1.0 },
                is_upside_down: false,
                is_baby: baby,
                is_in_water: false,
                has_red_overlay: true,
                pose: mod_api::Pose::Standing,
                bed_orientation: None,
            }
            .into();
            let frame: EntityRenderState = mod_api::__private::built::EntityRenderState {
                id: 42,
                partial: 0.5,
                age: 10.5,
                yaw: 0.0,
                pitch: 0.0,
                living: Some(living),
            }
            .into();
            renderer.observe(
                mod_api::EntityRef::new(mod_api::WorldEpoch::new(1), 42),
                10,
                false,
                Some(1.4),
                Ok(None),
            );
            let mut draw = Draw::default();
            renderer.submit(&frame, &mut draw).unwrap();
            assert_eq!(
                draw.sprite,
                if baby {
                    "naturalist:entity/deer/deer_baby"
                } else {
                    "naturalist:entity/deer/deer"
                }
            );
            assert_eq!(draw.has_antlers, !baby);
            assert_eq!(draw.tint, 0xFF7070);
        }
    }
    #[test]
    fn observed_appearance_drives_run_eat_and_texture() {
        let renderer = DeerRenderer::new();
        let id = mod_api::EntityRef::new(mod_api::WorldEpoch::new(4), 17);
        let state = DeerAppearance {
            variant: "example:woodland".into(),
            adult_texture: "naturalist:entity/deer/white_deer".into(),
            baby_texture: "naturalist:entity/deer/reindeer".into(),
            eating: true,
            sprinting: true,
            movement_speed: 0.31,
        };
        renderer.observe(id, 1, true, Some(1.4), Ok(Some(state.clone())));
        {
            let poses = renderer.poses.lock().unwrap();
            let pose = &poses[&17];
            assert_eq!(pose.appearance, Ok(Some(state)));
            assert_eq!(
                pose.clocks.each_ref().map(|c| c.factor),
                [0.5, 0.0, 0.0, 0.5]
            );
        }
        let living: mod_api::LivingRenderState = mod_api::__private::built::LivingRenderState {
            body_rot: 0.0,
            y_rot: 20.0,
            x_rot: 10.0,
            death_time: 0.0,
            walk_animation_pos: 0.0,
            walk_animation_speed: 0.5,
            scale: 1.0,
            age_scale: 1.0,
            is_upside_down: false,
            is_baby: false,
            is_in_water: false,
            has_red_overlay: false,
            pose: mod_api::Pose::Standing,
            bed_orientation: None,
        }
        .into();
        let frame: EntityRenderState = mod_api::__private::built::EntityRenderState {
            id: 17,
            partial: 0.5,
            age: 1.5,
            yaw: 0.0,
            pitch: 0.0,
            living: Some(living),
        }
        .into();
        let mut draw = Draw::default();
        renderer.submit(&frame, &mut draw).unwrap();
        assert_eq!(draw.sprite, "naturalist:entity/deer/white_deer");
        {
            let poses = renderer.poses.lock().unwrap();
            assert_eq!(poses[&17].clocks[0].phase_ms, 25);
            assert_eq!(poses[&17].clocks[3].phase_ms, 44);
        }
        renderer.observe(id, 1, true, Some(1.4), Err("bad appearance".into()));
        assert!(renderer.submit(&frame, &mut Draw::default()).is_err());
        let other_world = mod_api::EntityRef::new(mod_api::WorldEpoch::new(5), 17);
        renderer.observe(other_world, 2, false, Some(1.4), Ok(None));
        let poses = renderer.poses.lock().unwrap();
        assert_eq!(poses[&17].appearance, Ok(None));
        assert_eq!(poses[&17].handle, other_world);
    }
}
