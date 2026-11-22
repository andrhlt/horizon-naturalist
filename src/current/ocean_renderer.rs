//! Java-derived ocean wildlife models and keyframes on Horizon's entity API.
use mod_api::{BlockEntityDrawContext, EntityRenderState, EntityRenderer, ModelPart};
use std::{collections::HashMap, sync::Mutex};

static OCEAN_VARIANT: mod_api::AttachmentType<String> = mod_api::AttachmentType {
    namespace: "naturalist", name: "ocean_variant", default: || String::new(),
    codec: Some(mod_api::AttachmentCodec { encode: encode_variant, decode: decode_variant }),
    copy_on_death: true, sync: true,
};
fn encode_variant(value: &String) -> Result<mod_api::Compound, mod_api::AttachmentError> {
    Ok([("variant".into(), mod_api::Tag::String(value.as_str().into()))].into())
}
fn decode_variant(data: &mod_api::Compound) -> Result<String, mod_api::AttachmentError> {
    match data.get(&"variant".into()) {
        Some(mod_api::Tag::String(value)) => Ok(value.to_string_lossy()),
        _ => Err(mod_api::AttachmentError::Decode),
    }
}
pub(crate) fn record_variant(host: &mut dyn mod_api::ServerHost, entity: mod_api::EntityRef) {
    let variant = entity.nbt(host).and_then(|data| match data.get(&"Variant".into()) {
        Some(mod_api::Tag::String(value)) => Some(value.to_string_lossy()), _ => None,
    });
    if let Some(variant) = variant {
        if let Err(error) = OCEAN_VARIANT.set(host, mod_api::Holder::Entity(entity.id()), variant) {
            eprintln!("Naturalist: ocean variant sync failed: {error:?}");
        }
    }
}
pub(crate) fn synced_variant(host: &dyn mod_api::ClientHost, entity: mod_api::EntityRef) -> Option<String> {
    use mod_api::AttachmentQueries;
    host.query(&OCEAN_VARIANT).ok()?.get(mod_api::AttachmentHolder::Entity(entity)).ok().flatten()
}

#[derive(Clone, Copy)]
enum Target { Position, Rotation, Scale }
#[derive(Clone, Copy)]
enum Interpolation { Linear, CatmullRom, Mixed(&'static [usize]) }
struct Channel { bone: &'static str, target: Target, interpolation: Interpolation, keys: &'static [(f32, [f32; 3])] }
struct Clip { duration: f32, loops: bool, channels: &'static [Channel] }

fn bone<'a>(part: &'a mut ModelPart, name: &str) -> Option<&'a mut ModelPart> {
    for (child_name, child) in part.children_mut() {
        if child_name == name { return Some(child); }
        if let Some(found) = bone(child, name) { return Some(found); }
    }
    None
}
impl Clip {
    fn pose(&self, model: &mut ModelPart, time: f32) {
        let time = if self.loops && self.duration > 0.0 { time.rem_euclid(self.duration) } else { time };
        for channel in self.channels {
            if channel.keys.is_empty() { continue; }
            let Some(part) = bone(model, channel.bone) else { continue; };
            let end = channel.keys.partition_point(|(key, _)| *key < time).min(channel.keys.len()-1);
            let first = end.saturating_sub(1);
            let (t0, a) = channel.keys[first]; let (t1, b) = channel.keys[end];
            let u = if t1 == t0 { 0.0 } else { ((time-t0)/(t1-t0)).clamp(0.0,1.0) };
            let curved = match channel.interpolation { Interpolation::Linear => false, Interpolation::CatmullRom => true, Interpolation::Mixed(keys) => keys.contains(&end) };
            let v: [f32;3] = std::array::from_fn(|i| if curved {
                let p = channel.keys[first.saturating_sub(1)].1[i]; let q = channel.keys[(end+1).min(channel.keys.len()-1)].1[i];
                0.5*(2.0*a[i]+(-p+b[i])*u+(2.0*p-5.0*a[i]+4.0*b[i]-q)*u*u+(-p+3.0*a[i]-3.0*b[i]+q)*u*u*u)
            } else { a[i]+(b[i]-a[i])*u });
            match channel.target {
                Target::Position => part.offset_pos([v[0],-v[1],v[2]]),
                Target::Rotation => part.offset_rotation(v.map(f32::to_radians)),
                Target::Scale => part.offset_scale(v.map(|x| x-1.0)),
            }
        }
    }
}

#[allow(unused_variables, non_snake_case)]
#[path="ocean_assets/anglerfish_models.rs"] mod anglerfish_models;
#[allow(dead_code)]
#[path="ocean_assets/anglerfish_animations.rs"] mod anglerfish_animations;
#[allow(unused_variables, non_snake_case)]
#[path="ocean_assets/blobfish_models.rs"] mod blobfish_models;
#[allow(dead_code)]
#[path="ocean_assets/blobfish_animations.rs"] mod blobfish_animations;
#[allow(unused_variables, non_snake_case)]
#[path="ocean_assets/giant_isopod_models.rs"] mod giant_isopod_models;
#[allow(dead_code)]
#[path="ocean_assets/giant_isopod_animations.rs"] mod giant_isopod_animations;
#[allow(unused_variables, non_snake_case)]
#[path="ocean_assets/great_white_shark_models.rs"] mod shark_models;
#[allow(dead_code)]
#[path="ocean_assets/great_white_shark_animations.rs"] mod shark_animations;
#[allow(unused_variables, non_snake_case)]
#[path="ocean_assets/jellyfish_models.rs"] mod jellyfish_models;
#[allow(dead_code)]
#[path="ocean_assets/jellyfish_animations.rs"] mod jellyfish_animations;
#[allow(unused_variables, non_snake_case)]
#[path="ocean_assets/ray_models.rs"] mod ray_models;
#[allow(dead_code)]
#[path="ocean_assets/ray_animations.rs"] mod ray_animations;
#[allow(unused_variables, non_snake_case)]
#[path="ocean_assets/starfish_models.rs"] mod starfish_models;
#[allow(dead_code)]
#[path="ocean_assets/starfish_animations.rs"] mod starfish_animations;
#[allow(unused_variables, non_snake_case)]
#[path="ocean_assets/whale_models.rs"] mod whale_models;
#[allow(dead_code)]
#[path="ocean_assets/whale_animations.rs"] mod whale_animations;

pub(crate) struct OceanRenderer { model: ModelPart, baby_model: Option<ModelPart>, texture: &'static str, species: &'static str, animation: &'static Clip, variants: Mutex<HashMap<u32, String>> }
impl OceanRenderer {
    fn new(species: &'static str, model: ModelPart, texture: &'static str, animation: &'static Clip) -> Self { Self { species, model, baby_model: None, texture, animation, variants: Mutex::new(HashMap::new()) } }
    pub(crate) fn anglerfish() -> Self { Self::new("anglerfish", anglerfish_models::adult(), "naturalist:entity/anglerfish/red", &anglerfish_animations::ADULT_SWIM) }
    pub(crate) fn blobfish() -> Self { Self::new("blobfish", blobfish_models::gray(), "naturalist:entity/blobfish/gray", &blobfish_animations::ADULT_GRAY_SWIM) }
    pub(crate) fn isopod() -> Self { Self::new("giant_isopod", giant_isopod_models::adult(), "naturalist:entity/giant_isopod/brown", &giant_isopod_animations::ADULT_WALK) }
    pub(crate) fn shark() -> Self { Self::new("great_white_shark", shark_models::adult(), "naturalist:entity/great_white_shark", &shark_animations::ADULT_SWIM) }
    pub(crate) fn jellyfish() -> Self { Self::new("jellyfish", jellyfish_models::adult(), "naturalist:entity/jellyfish/white", &jellyfish_animations::ADULT_SWIM) }
    pub(crate) fn ray() -> Self { Self::new("ray", ray_models::adult(), "naturalist:entity/ray/eagle_ray", &ray_animations::ADULT_SWIM_IDLE_EVENT) }
    pub(crate) fn starfish() -> Self { Self::new("starfish", starfish_models::adult(), "naturalist:entity/starfish/orange", &starfish_animations::ADULT_IDLE) }
    pub(crate) fn whale() -> Self { let mut r = Self::new("whale", whale_models::adult(), "naturalist:entity/whale/whale", &whale_animations::ADULT_SWIM); r.baby_model = Some(whale_models::baby()); r }
    pub(crate) fn observe_variant(&self, id: u32, variant: Option<String>) {
        let mut variants = self.variants.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(variant) = variant { variants.insert(id, variant); } else { variants.remove(&id); }
    }
    pub(crate) fn retain_live(&self, host: &dyn mod_api::ClientHost) {
        self.variants.lock().unwrap_or_else(std::sync::PoisonError::into_inner).retain(|&id, _| host.entity(id).is_some());
    }
    pub(crate) fn clear(&self) { self.variants.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clear(); }
    fn texture(&self, id: u32) -> String {
        let variants = self.variants.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let name = variants.get(&id).map(|v| v.rsplit(':').next().unwrap_or(v.as_str()));
        let file = match (self.species, name) {
            ("anglerfish", Some("glow")) => "anglerfish/glow",
            ("anglerfish", _) => "anglerfish/red",
            ("blobfish", Some("pink")) => "blobfish/pink",
            ("giant_isopod", Some("blue")) => "giant_isopod/blue",
            ("jellyfish", Some(color @ ("blue" | "green" | "orange" | "pink"))) => return format!("naturalist:entity/jellyfish/{color}"),
            ("ray", Some("mobula_ray")) => "ray/mobula_ray",
            ("ray", Some("stingray")) => "ray/stingray",
            ("starfish", Some(color @ ("blue" | "purple" | "red"))) => return format!("naturalist:entity/starfish/{color}"),
            ("whale", _) if name == Some("whale_baby") => "whale/whale_baby",
            ("blobfish", _) => "blobfish/gray",
            ("giant_isopod", _) => "giant_isopod/brown",
            ("jellyfish", _) => "jellyfish/white",
            ("ray", _) => "ray/eagle_ray",
            ("starfish", _) => "starfish/orange",
            ("whale", _) => "whale/whale",
            _ => return self.texture.to_owned(),
        };
        format!("naturalist:entity/{file}")
    }
}
impl EntityRenderer for OceanRenderer {
    fn submit(&self, entity: &EntityRenderState, draw: &mut dyn BlockEntityDrawContext) -> Result<(), String> {
        let Some(living) = entity.living.as_ref() else { return Ok(()); };
        let mut model = if living.is_baby { self.baby_model.as_ref().unwrap_or(&self.model).clone() } else { self.model.clone() };
        self.animation.pose(&mut model, entity.age.max(0.0) * 0.05 * (living.walk_animation_speed / 0.25).clamp(0.4, 2.0));
        draw.scale([living.scale; 3]);
        draw.rotate_about([0.0,1.0,0.0], 180.0-living.body_rot, [0.0;3]);
        draw.scale([1.0,-1.0,-1.0]);
        draw.translate([0.0,-1.501,0.0]);
        let texture = self.texture(entity.id);
        if !draw.model_part(&model, &texture, if living.has_red_overlay { 0xFF7070 } else { 0xFFFFFF }) { return Err(format!("missing ocean wildlife texture {texture}")); }
        Ok(())
    }
}
