//! Current-runtime renderers for the terrestrial wildlife covered by APP-2126.
use mod_api::{BlockEntityDrawContext, EntityRenderState, EntityRenderer, ModelPart};

#[path = "../bear_models.rs"] mod bear_models;
#[path = "../black_bear_models.rs"] mod black_bear_models;
#[path = "../boar_models.rs"] mod boar_models;
#[path = "../snake_models.rs"] mod snake_models;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Kind { Bear, BlackBear, Boar, Snake }

pub(crate) struct WildlifeRenderer { kind: Kind, adult: ModelPart, baby: Option<ModelPart> }

fn bone<'a>(part: &'a mut ModelPart, name: &str) -> Option<&'a mut ModelPart> {
    for (child_name, child) in part.children_mut() {
        if child_name == name { return Some(child); }
        if let Some(found) = bone(child, name) { return Some(found); }
    }
    None
}

/// Apply the source animals' locomotion cue to the current living render state.
/// The current API does not expose Naturalist's animation controller, but it
/// does provide the same interpolated walk position/speed used by vanilla
/// living renderers.  Keep this adapter deterministic and model-local.
fn animate(model: &mut ModelPart, kind: Kind, position: f32, speed: f32) {
    let swing = position.sin() * speed.clamp(0.0, 1.0) * 0.65;
    let names: &[(&str, f32)] = match kind {
        Kind::Bear | Kind::BlackBear => &[("leftArm", 1.0), ("rightArm", -1.0), ("leftLeg", -1.0), ("rightLeg", 1.0)],
        Kind::Boar => &[("leftArm", 1.0), ("rightArm", -1.0), ("leftLeg", -1.0), ("rightLeg", 1.0)],
        Kind::Snake => &[("tail", 0.35), ("tail2", 0.55), ("tail3", 0.8), ("tail4", 1.0)],
    };
    for &(name, phase) in names {
        if let Some(part) = bone(model, name) {
            part.offset_rotation([swing * phase, 0.0, 0.0]);
        }
    }
}

impl WildlifeRenderer {
    fn new(kind: Kind, adult: ModelPart, baby: Option<ModelPart>) -> Self { Self { kind, adult, baby } }
    pub(crate) fn bear() -> Self { Self::new(Kind::Bear, bear_models::adult(), Some(bear_models::baby())) }
    pub(crate) fn black_bear() -> Self { Self::new(Kind::BlackBear, black_bear_models::adult(), Some(black_bear_models::baby())) }
    pub(crate) fn boar() -> Self { Self::new(Kind::Boar, boar_models::adult(), Some(boar_models::baby())) }
    pub(crate) fn snake() -> Self { Self::new(Kind::Snake, snake_models::adult(), None) }
    fn texture(&self, baby: bool) -> &'static str {
        match (self.kind, baby) {
            (Kind::Bear, false) => "naturalist:entity/bear/bear",
            (Kind::Bear, true) => "naturalist:entity/bear/bear_baby",
            (Kind::BlackBear, false) => "naturalist:entity/black_bear/black_bear",
            (Kind::BlackBear, true) => "naturalist:entity/black_bear/black_bear_baby",
            (Kind::Boar, false) => "naturalist:entity/boar/boar",
            (Kind::Boar, true) => "naturalist:entity/boar/boar_baby",
            (Kind::Snake, _) => "naturalist:entity/snake/snake",
        }
    }
}

impl EntityRenderer for WildlifeRenderer {
    fn submit(&self, entity: &EntityRenderState, draw: &mut dyn BlockEntityDrawContext) -> Result<(), String> {
        let Some(living) = entity.living else { return Ok(()) };
        let mut model = if living.is_baby { self.baby.as_ref().unwrap_or(&self.adult).clone() } else { self.adult.clone() };
        animate(&mut model, self.kind, living.walk_animation_pos, living.walk_animation_speed);
        draw.rotate_about([0.0, 1.0, 0.0], entity.yaw, [0.0; 3]);
        draw.rotate_about([1.0, 0.0, 0.0], entity.pitch, [0.0; 3]);
        draw.scale([living.scale, -living.scale, -living.scale]);
        draw.translate([0.0, -1.501 * living.age_scale, 0.0]);
        let tint = if living.has_red_overlay { 0xFF7070 } else { 0xFFFFFF };
        if !draw.model_part(&model, self.texture(living.is_baby), tint) {
            return Err(format!("missing Naturalist {:?} texture", self.kind));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constructors_cover_app_2126_models() {
        let _ = [WildlifeRenderer::bear(), WildlifeRenderer::black_bear(), WildlifeRenderer::boar(), WildlifeRenderer::snake()];
    }
    #[test]
    fn locomotion_animation_handles_all_source_bone_sets() {
        let models = [
            (Kind::Bear, bear_models::adult()),
            (Kind::BlackBear, black_bear_models::adult()),
            (Kind::Boar, boar_models::adult()),
            (Kind::Snake, snake_models::adult()),
        ];
        for (kind, mut model) in models { animate(&mut model, kind, 1.25, 0.8); }
    }
}
