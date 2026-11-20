// Deer behavior is staged with the engine; this module draws the original model layers.
use super::{Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use horizon_core::random::RandomSource;
use horizon_world::loot::PackOverlay;
use serde::Deserialize;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    path::Path,
    time::{Duration, Instant},
};

const DEFAULT_VARIANT: &str = "naturalist:deer";
const ADULT_TEXTURE: &str = "naturalist:entity/deer/deer";
const FAWN_TEXTURE: &str = "naturalist:entity/deer/deer_baby";
const REGISTRY: &str = "naturalist/deer_variant";

#[derive(Deserialize)]
#[serde(untagged)]
enum BiomeSelector {
    One(String),
    Many(Vec<String>),
}

impl BiomeSelector {
    fn matches(&self, game: &Game<'_>, name: &str) -> bool {
        match self {
            Self::One(id) => game.biome_matches(id, name),
            Self::Many(ids) => ids.iter().any(|id| game.biome_matches(id, name)),
        }
    }
}

#[derive(Deserialize)]
struct Variant {
    texture: String,
    baby_texture: Option<String>,
    biomes: Option<BiomeSelector>,
    #[serde(default = "unit_weight")]
    weight: u32,
    #[serde(default)]
    priority: i32,
}

fn unit_weight() -> u32 {
    1
}

#[derive(Default)]
pub(super) struct DeerVariants(BTreeMap<String, Variant>);

impl DeerVariants {
    pub(super) fn load(root: &Path, packs: &PackOverlay<'_>) -> Self {
        let mut variants = BTreeMap::new();
        for namespace in horizon_client::mod_content::namespaces(root) {
            let path = root.join("data").join(&namespace).join(REGISTRY);
            if let Ok(folder) = horizon_world::json_folder::read(&path) {
                for file in &folder.files {
                    if let Ok(bytes) = file.bytes()
                        && let Ok(variant) = serde_json::from_slice(&bytes)
                    {
                        variants.insert(format!("{namespace}:{}", file.name), variant);
                    }
                }
            }
        }
        for (id, files) in packs.kind(REGISTRY) {
            match serde_json::from_slice(PackOverlay::top(files)) {
                Ok(variant) => {
                    variants.insert(id.to_owned(), variant);
                }
                Err(e) => log::warn!("Naturalist: deer variant {id}: {e}"),
            }
        }
        Self(variants)
    }

    fn choose(&self, biome: &str, game: &Game<'_>, random: &mut impl RandomSource) -> Option<String> {
        let mut pool: Vec<_> = self.0.iter().filter(|(_, v)| {
            v.weight > 0 && v.biomes.as_ref().is_some_and(|b| b.matches(game, biome))
        }).collect();
        if pool.is_empty() {
            pool.extend(self.0.iter().filter(|(_, v)| v.weight > 0 && v.biomes.is_none()));
        }
        let priority = pool.iter().map(|(_, v)| v.priority).max()?;
        pool.retain(|(_, v)| v.priority == priority);
        let total = pool.iter().try_fold(0i32, |total, (_, v)| {
            total.checked_add(i32::try_from(v.weight).ok()?)
        })?;
        let mut roll = u32::try_from(random.next_int_bound(total).ok()?).ok()?;
        for (id, variant) in pool {
            if roll < variant.weight {
                return Some(id.clone());
            }
            roll -= variant.weight;
        }
        None
    }

    fn texture(&self, id: &str, baby: bool) -> Option<&str> {
        let variant = self.0.get(id)?;
        Some(if baby {
            variant.baby_texture.as_deref().unwrap_or(&variant.texture)
        } else {
            &variant.texture
        })
    }
}

#[path = "deer_models.rs"]
#[allow(unused_variables, non_snake_case)]
mod deer_models;
#[path = "deer_animations.rs"]
mod deer_animations;
use deer_animations::*;

#[derive(Clone, Copy)]
pub(super) enum Target { Position, Rotation, Scale }

pub(super) enum Interpolation {
    Linear,
    CatmullRom,
    Mixed(&'static [usize]),
}

impl Interpolation {
    fn catmull_rom(&self, ending_key: usize) -> bool {
        match self {
            Self::Linear => false,
            Self::CatmullRom => true,
            Self::Mixed(keys) => keys.contains(&ending_key),
        }
    }
}

pub(super) struct Channel {
    pub(super) bone: &'static str,
    pub(super) target: Target,
    pub(super) interpolation: Interpolation,
    pub(super) keys: &'static [(f32, [f32; 3])],
}

pub(super) struct Clip {
    pub(super) duration: f32,
    pub(super) loops: bool,
    pub(super) channels: &'static [Channel],
}

impl Clip {
    pub(super) fn pose(&self, model: &mut ModelPart, time: f32, weight: f32) {
        let time = if self.loops && self.duration > 0.0 {
            time.rem_euclid(self.duration)
        } else { time };
        for channel in self.channels {
            let Some(part) = bone(model, channel.bone) else { continue };
            let end = channel.keys.partition_point(|(key, _)| *key < time)
                .min(channel.keys.len() - 1);
            let first = end.saturating_sub(1);
            let (start_time, start) = channel.keys[first];
            let (end_time, end_value) = channel.keys[end];
            let blend = if end_time == start_time { 0.0 }
                else { ((time - start_time) / (end_time - start_time)).clamp(0.0, 1.0) };
            let xyz = if channel.interpolation.catmull_rom(end) {
                let before = channel.keys[first.saturating_sub(1)].1;
                let after = channel.keys[(end + 1).min(channel.keys.len() - 1)].1;
                std::array::from_fn(|i| {
                    let [a, b, c, d] = [before[i], start[i], end_value[i], after[i]];
                    0.5 * (2.0 * b + (-a + c) * blend
                        + (2.0 * a - 5.0 * b + 4.0 * c - d) * blend * blend
                        + (-a + 3.0 * b - 3.0 * c + d) * blend * blend * blend)
                })
            } else {
                std::array::from_fn(|i| start[i] + (end_value[i] - start[i]) * blend)
            };
            let [x, y, z] = xyz;
            match channel.target {
                Target::Position => part.offset_pos([x * weight, -y * weight, z * weight]),
                Target::Rotation => part.offset_rotation(xyz.map(|angle| angle.to_radians() * weight)),
                Target::Scale => part.offset_scale(xyz.map(|scale| (scale - 1.0) * weight)),
            }
        }
    }
}

#[derive(Default)]
pub(super) struct AnimationClock {
    previous_factor: f32,
    factor: f32,
    phase_ms: i64,
    last_ms: Option<i64>,
}
impl AnimationClock {
    fn advance(&mut self, active: bool, tick: i32) {
        self.advance_with_rate(active, tick, 0.5);
    }

    pub(super) fn advance_with_rate(&mut self, active: bool, tick: i32, rate: f32) {
        self.previous_factor = self.factor;
        let target = if active { 1.0 } else { 0.0 };
        self.factor = (self.factor + (target - self.factor) * rate).clamp(0.0, 1.0);
        if active {
            if self.last_ms.is_none() {
                self.last_ms = Some(i64::from(tick) * 50);
                self.phase_ms = 0;
            }
        } else if self.factor < 0.001 {
            self.last_ms = None;
        }
    }

    pub(super) fn draw(&mut self, clip: &Clip, model: &mut ModelPart, age: f32, partial: f32, speed: f32, scale: f32) {
        let Some((phase, weight)) = self.sample(age, partial, speed, scale) else { return; };
        clip.pose(model, phase, weight);
    }

    pub(super) fn started_phase_ms(&self) -> Option<i64> {
        self.last_ms.map(|_| self.phase_ms)
    }

    pub(super) fn factor_at(&self, partial: f32) -> f32 {
        self.previous_factor + partial * (self.factor - self.previous_factor)
    }

    fn sample(&mut self, age: f32, partial: f32, speed: f32, scale: f32) -> Option<(f32, f32)> {
        let weight = self.factor_at(partial) * scale;
        if weight <= 0.05 {
            return None;
        }
        if let Some(last) = &mut self.last_ms {
            let now = (age * 1000.0 / 20.0).floor() as i64;
            self.phase_ms += ((now - *last) as f32 * speed) as i64;
            *last = now;
        }
        Some((self.phase_ms as f32 / 1000.0, weight))
    }
}

struct DeerPose {
    clocks: [AnimationClock; 4],
    tick: i32,
    last_seen: Instant,
}


thread_local! {
    static ADULT: RefCell<ModelPart> = RefCell::new(deer_models::adult());
    static BABY: RefCell<ModelPart> = RefCell::new(deer_models::baby());
    static POSES: RefCell<HashMap<u32, DeerPose>> = RefCell::new(HashMap::new());
}

impl DeerPose {
    fn new(now: Instant) -> Self {
        Self { clocks: Default::default(), tick: -1, last_seen: now }
    }

    fn advance(&mut self, mob: &Mob, now: Instant) {
        let tick = mob.age as i32;
        if tick < self.tick {
            *self = Self::new(now);
        }
        if tick > self.tick {
            let moving = mob.velocity.x * mob.velocity.x + mob.velocity.z * mob.velocity.z > 1.0e-6;
            let eating = mob.extra.anim_ticks > 0;
            let sprinting = mob.extra.sprinting;
            for (clock, active) in self.clocks.iter_mut()
                .zip([eating, !moving, moving && !sprinting, moving && sprinting])
            {
                clock.advance(active, tick);
            }
            self.tick = tick;
        }
        self.last_seen = now;
    }
}

fn deer_pose(poses: &mut HashMap<u32, DeerPose>, id: u32, now: Instant) -> &mut DeerPose {
    if !poses.contains_key(&id) {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(60));
    }
    poses.entry(id).or_insert_with(|| DeerPose::new(now))
}

pub(super) fn bone<'a>(part: &'a mut ModelPart, name: &str) -> Option<&'a mut ModelPart> {
    for (child_name, child) in part.children_mut() {
        if child_name == name { return Some(child); }
        if let Some(found) = bone(child, name) { return Some(found); }
    }
    None
}

impl Game<'_> {
    pub(super) fn finalize_deer_variant(&self, mob: &mut Mob) {
        if mob.kind.id != DEFAULT_VARIANT {
            return;
        }
        let position = mob.position;
        let biome = self.biome_at([position.x, position.y, position.z]);
        if let Some(tables) = &self.world_tables {
            mob.extra.naturalist_deer_variant =
                tables.deer_variants.choose(biome, self, &mut mob.random);
        }
    }

    pub(super) fn update_deer_herds(&mut self) {
        use horizon_core::geometry::Vec3;
        let callers: Vec<_> = self.mobs.iter_mut().filter_map(|mob| {
            mob.extra.deer_panic_relay.take().map(|ticks| {
                (mob.bounds().inflate(Vec3::new(8.0, 4.0, 8.0)), ticks)
            })
        }).collect();
        for mob in self.mobs.iter_mut().filter(|mob| mob.kind.id == "naturalist:deer") {
            for (range, ticks) in &callers {
                if range.intersects(mob.bounds()) {
                    mob.extra.deer_panic_ticks = *ticks;
                }
            }
            if mob.extra.deer_panic_ticks >= 0 {
                mob.extra.deer_panic_ticks -= 1;
                if mob.extra.deer_panic_ticks == 0 {
                    mob.ai.last_hurt_by = None;
                    mob.ai.last_hurt_by_ref = None;
                }
            }
        }
    }

    pub(super) fn deer_texture<'a>(&'a self, mob: &'a Mob, baby: bool) -> &'a str {
        let remote = if baby { &mob.extra.naturalist_fawn_texture }
            else { &mob.extra.naturalist_deer_texture };
        let variant = mob.extra.naturalist_deer_variant.as_deref().unwrap_or(DEFAULT_VARIANT);
        remote.as_deref()
            .or_else(|| self.world_tables.as_ref()
                .and_then(|tables| tables.deer_variants.texture(variant, baby)))
            .unwrap_or(if baby && variant == DEFAULT_VARIANT { FAWN_TEXTURE } else { ADULT_TEXTURE })
    }

    pub(super) fn tick_naturalist_deer_animations(&self) {
        if self.remote.is_none() {
            return;
        }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| !mob.removed && mob.kind.id == "naturalist:deer") {
                deer_pose(poses, mob.id, now).advance(mob, now);
            }
        });
    }

    pub(super) fn draw_naturalist_deer(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        if render.kind != "naturalist:deer" {
            return false;
        }
        let model = if render.baby { &BABY } else { &ADULT };
        let texture = self.deer_texture(mob, render.baby);
        model.with_borrow_mut(|part| {
            part.reset_all_poses();
            for name in ["sleep", "saddle"] {
                if let Some(hidden) = bone(part, name) { hidden.visible = false; }
            }
            let eating = mob.extra.anim_ticks > 0;
            let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
            let speed = (render.walk_speed / gait).clamp(0.4, 2.0);
            let (clips, rates) = if render.baby {
                ([&BABY_EAT, &BABY_IDLE, &BABY_WALK, &BABY_RUN], [1.0, 1.5, 1.7 * speed, speed])
            } else {
                ([&ADULT_EAT, &ADULT_IDLE, &ADULT_WALK, &ADULT_RUN], [1.0, 1.0, 2.4 * speed, 2.3 * speed])
            };
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = deer_pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.advance(mob, now);
                }
                let weights = [1.0, 1.0 - (render.walk_speed * 2.5).min(1.0), 1.0, 1.0];
                for i in 0..4 {
                    pose.clocks[i].draw(clips[i], part, render.age, render.partial, rates[i], weights[i]);
                }
            });
            if !eating && let Some(neck) = bone(part, "neck") {
                neck.x_rot += render.pitch.to_radians();
                neck.y_rot += render.head_yaw.to_radians();
            }
            let scale = mob.model_scale();
            draw.scale([scale, -scale, -scale]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(draw, part, texture, if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF })
        })
    }
}
