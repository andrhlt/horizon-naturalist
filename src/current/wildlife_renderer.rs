//! Current-runtime renderers for the terrestrial wildlife covered by APP-2126.
use mod_api::{BlockEntityDrawContext, EntityRenderState, EntityRenderer, ModelPart};

#[path = "../bear_models.rs"] mod bear_models;
#[path = "../black_bear_models.rs"] mod black_bear_models;
#[path = "../boar_models.rs"] mod boar_models;
#[path = "../snake_models.rs"] mod snake_models;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Kind { Bear, BlackBear, Boar, Snake }

pub(crate) struct WildlifeRenderer { kind: Kind, adult: ModelPart, baby: Option<ModelPart> }

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
        let model = if living.is_baby { self.baby.as_ref().unwrap_or(&self.adult) } else { &self.adult };
        draw.rotate_about([0.0, 1.0, 0.0], entity.yaw, [0.0; 3]);
        draw.rotate_about([1.0, 0.0, 0.0], entity.pitch, [0.0; 3]);
        draw.scale([living.scale, -living.scale, -living.scale]);
        draw.translate([0.0, -1.501 * living.age_scale, 0.0]);
        let tint = if living.has_red_overlay { 0xFF7070 } else { 0xFFFFFF };
        if !draw.model_part(model, self.texture(living.is_baby), tint) {
            return Err(format!("missing Naturalist {:?} texture", self.kind));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests { use super::*; #[test] fn constructors_cover_app_2126_models() { let _ = [WildlifeRenderer::bear(), WildlifeRenderer::black_bear(), WildlifeRenderer::boar(), WildlifeRenderer::snake()]; } }
