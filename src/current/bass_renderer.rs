//! Bass models, keyframes, and interpolation adapted from Naturalist 2.0.7 (MIT).

use crate::bass_appearance::BassAppearance;
use mod_api::{BlockEntityDrawContext, EntityRenderState, EntityRenderer, ModelPart};
use std::{collections::HashMap, sync::Mutex};
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

#[path = "bass_animations.rs"]
mod animations;
#[path = "bass_models.rs"]
#[allow(unused_variables, non_snake_case)]
mod models;
struct Pose {
    handle: mod_api::EntityRef,
    tick: i32,
    clocks: [AnimationClock; 2],
    previous: [f32; 2],
    current: [f32; 2],
    appearance: Result<Option<BassAppearance>, String>,
    height: Option<f32>,
}
impl Pose {
    fn new(handle: mod_api::EntityRef) -> Self {
        Self {
            handle,
            tick: -1,
            clocks: Default::default(),
            previous: [0.0; 2],
            current: [0.0; 2],
            appearance: Ok(None),
            height: None,
        }
    }
}
pub(crate) struct BassRenderer {
    models: [ModelPart; 3],
    poses: Mutex<HashMap<u32, Pose>>,
}
impl BassRenderer {
    pub(crate) fn new() -> Self {
        Self {
            models: [models::adult(), models::medium(), models::large()],
            poses: Mutex::new(HashMap::new()),
        }
    }
    pub(crate) fn observe(
        &self,
        handle: mod_api::EntityRef,
        tick: i32,
        appearance: Result<Option<BassAppearance>, String>,
        height: Option<f32>,
    ) {
        let mut poses = self
            .poses
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let p = poses
            .entry(handle.id())
            .or_insert_with(|| Pose::new(handle));
        if p.handle != handle || tick < p.tick {
            *p = Pose::new(handle);
        }
        if tick > p.tick {
            if let Ok(Some(a)) = &appearance {
                p.clocks[0].advance(!a.in_water, tick);
                p.clocks[1].advance(a.in_water, tick);
                p.previous = p.current;
                p.current = [a.tilt, a.pitch];
            }
            p.tick = tick;
        }
        p.appearance = appearance;
        p.height = height;
    }
    pub(crate) fn retain_live(&self, host: &dyn mod_api::ClientHost) {
        self.poses
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .retain(|_, p| p.handle.position(host).is_some());
    }
    pub(crate) fn clear(&self) {
        self.poses
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear();
    }
}
impl EntityRenderer for BassRenderer {
    fn submit(
        &self,
        entity: &EntityRenderState,
        draw: &mut dyn BlockEntityDrawContext,
    ) -> Result<(), String> {
        let Some(living) = entity.living.as_ref() else {
            return Ok(());
        };
        let mut poses = self
            .poses
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let state = match poses.get(&entity.id) {
            Some(p) => p
                .appearance
                .clone()
                .map_err(|e| format!("bass appearance: {e}"))?,
            None => None,
        };
        let fallback = BassAppearance::default();
        let appearance = state.as_ref().unwrap_or(&fallback);
        let tier = match appearance.variant.as_str() {
            "naturalist:bass_medium" => 1,
            "naturalist:bass_large" => 2,
            _ => 0,
        };
        let mut model = self.models[tier].clone();
        let mut tilt = 0.0;
        if let Some(p) = poses.get_mut(&entity.id) {
            let gait = (living.walk_animation_speed / 0.25).clamp(0.4, 2.0);
            p.clocks[0].draw(
                &animations::ADULT_FLOP,
                &mut model,
                entity.age,
                entity.partial,
                1.0,
                1.0,
            );
            p.clocks[1].draw(
                &animations::ADULT_SWIM,
                &mut model,
                entity.age,
                entity.partial,
                gait,
                1.0,
            );
            let interp = |i: usize| p.previous[i] + (p.current[i] - p.previous[i]) * entity.partial;
            if let Some(body) = bone(&mut model, if tier == 2 { "root" } else { "body" }) {
                body.x_rot += interp(1).to_radians();
            }
            tilt = interp(0);
        }
        let height = poses.get(&entity.id).and_then(|p| p.height);
        drop(poses);
        draw.scale([living.scale; 3]);
        draw.rotate_about([0.0, 1.0, 0.0], 180.0 - living.body_rot, [0.0; 3]);
        if living.death_time > 0.0 {
            let progress = (((living.death_time - 1.0) / 20.0 * 1.6).max(0.0))
                .sqrt()
                .min(1.0);
            draw.rotate_about([0.0, 0.0, 1.0], progress * 90.0, [0.0; 3]);
        } else if living.is_upside_down
            && let Some(height) = height
        {
            draw.translate([0.0, height + 0.1, 0.0]);
            draw.rotate_about([0.0, 0.0, 1.0], 180.0, [0.0; 3]);
        }
        draw.rotate_about([0.0, 0.0, 1.0], -tilt, [0.0; 3]);
        draw.scale([1.0, -1.0, -1.0]);
        draw.translate([0.0, -1.501, 0.0]);
        if !draw.model_part(
            &model,
            &appearance.texture,
            if living.has_red_overlay {
                0xFF7070
            } else {
                0xFFFFFF
            },
        ) {
            return Err(format!("missing bass texture {}", appearance.texture));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Default)]
    struct Draw {
        texture: String,
        root_y: f32,
        red: u32,
        rolls: Vec<f32>,
    }
    impl BlockEntityDrawContext for Draw {
        fn rotate_about(&mut self, axis: [f32; 3], degrees: f32, _: [f32; 3]) {
            if axis == [0.0, 0.0, 1.0] {
                self.rolls.push(degrees);
            }
        }
        fn scale(&mut self, _: [f32; 3]) {}
        fn translate(&mut self, _: [f32; 3]) {}
        fn model_part(&mut self, model: &ModelPart, texture: &str, tint: u32) -> bool {
            self.texture = texture.into();
            self.red = tint;
            self.root_y = bone(&mut model.clone(), "root").unwrap().y;
            true
        }
    }
    fn frame(id: u32) -> EntityRenderState {
        let living: mod_api::LivingRenderState = mod_api::__private::built::LivingRenderState {
            body_rot: 0.0,
            y_rot: 0.0,
            x_rot: 0.0,
            death_time: 0.0,
            walk_animation_pos: 0.0,
            walk_animation_speed: 0.25,
            scale: 1.0,
            age_scale: 1.0,
            is_upside_down: false,
            is_baby: false,
            is_in_water: true,
            has_red_overlay: true,
            pose: mod_api::Pose::Standing,
            bed_orientation: None,
        }
        .into();
        mod_api::__private::built::EntityRenderState {
            id,
            partial: 0.5,
            age: 2.5,
            yaw: 0.0,
            pitch: 0.0,
            living: Some(living),
        }
        .into()
    }
    #[test]
    fn original_mesh_tiers_and_synced_animation_are_selected() {
        let r = BassRenderer::new();
        let id = mod_api::EntityRef::new(mod_api::WorldEpoch::new(1), 7);
        for (variant, y) in [
            ("naturalist:bass", 22.0),
            ("naturalist:bass_medium", 21.0),
            ("naturalist:bass_large", 20.25),
        ] {
            let state = BassAppearance {
                variant: variant.into(),
                texture: "test:fish".into(),
                in_water: true,
                tilt: 8.0,
                pitch: 10.0,
            };
            r.observe(id, 1, Ok(Some(state.clone())), Some(1.0));
            r.observe(id, 2, Ok(Some(state)), Some(1.0));
            let mut draw = Draw::default();
            r.submit(&frame(7), &mut draw).unwrap();
            assert_eq!(draw.root_y, y);
            assert_eq!(draw.texture, "test:fish");
            assert_eq!(draw.red, 0xFF7070);
            assert!(draw.rolls.contains(&-8.0));
        }
        let poses = r.poses.lock().unwrap();
        assert!(poses[&7].clocks[1].factor > 0.0);
        assert_eq!(poses[&7].clocks[0].factor, 0.0);
    }
    #[test]
    fn original_swim_and_flop_keyframes_pose_the_expected_bones() {
        let mut swim = models::adult();
        animations::ADULT_SWIM.pose(&mut swim, 0.125, 1.0);
        assert!((bone(&mut swim, "tail").unwrap().y_rot.to_degrees() + 45.0).abs() < 1.0e-4);
        let mut flop = models::adult();
        animations::ADULT_FLOP.pose(&mut flop, 0.25, 1.0);
        assert!((bone(&mut flop, "tail").unwrap().y_rot.to_degrees() - 22.5).abs() < 1.0e-4);
    }
    #[test]
    fn world_reuse_discards_stale_state_and_bad_data_is_reported() {
        let r = BassRenderer::new();
        let id = mod_api::EntityRef::new(mod_api::WorldEpoch::new(1), 7);
        r.observe(id, 1, Err("decode failed".into()), None);
        assert!(r.submit(&frame(7), &mut Draw::default()).is_err());
        let next = mod_api::EntityRef::new(mod_api::WorldEpoch::new(2), 7);
        r.observe(next, 2, Ok(None), None);
        let mut draw = Draw::default();
        r.submit(&frame(7), &mut draw).unwrap();
        assert_eq!(draw.texture, "naturalist:entity/bass");
        assert_eq!(r.poses.lock().unwrap()[&7].handle, next);
    }
}
