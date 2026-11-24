//! Naturalist 2.0.7 models and basic movement poses for freshwater species.
use mod_api::{BlockEntityDrawContext, EntityRenderState, EntityRenderer, ModelPart};

#[path = "alligator_models.rs"]
mod alligator_models;
#[path = "capybara_models.rs"]
mod capybara_models;
#[path = "catfish_models.rs"]
mod catfish_models;
#[path = "clam_models.rs"]
mod clam_models;
#[path = "hippo_models.rs"]
mod hippo_models;
#[path = "piranha_models.rs"]
mod piranha_models;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Species {
    Alligator,
    Capybara,
    Catfish,
    Clam,
    Hippo,
    Piranha,
}

pub(crate) struct WildlifeRenderer {
    species: Species,
    adult: ModelPart,
    baby: Option<ModelPart>,
}

impl WildlifeRenderer {
    pub(crate) fn new(species: Species) -> Self {
        let (adult, baby) = match species {
            Species::Alligator => (alligator_models::adult(), Some(alligator_models::baby())),
            Species::Capybara => (capybara_models::adult(), Some(capybara_models::baby())),
            Species::Catfish => (catfish_models::adult(), None),
            Species::Clam => (clam_models::adult(), None),
            Species::Hippo => (hippo_models::adult(), Some(hippo_models::baby())),
            Species::Piranha => (piranha_models::adult(), None),
        };
        Self { species, adult, baby }
    }

    fn texture(&self, baby: bool) -> &'static str {
        match (self.species, baby) {
            (Species::Alligator, true) => "naturalist:entity/alligator/alligator_baby",
            (Species::Alligator, false) => "naturalist:entity/alligator/alligator",
            (Species::Capybara, true) => "naturalist:entity/capybara/capybara_baby",
            (Species::Capybara, false) => "naturalist:entity/capybara/capybara",
            (Species::Catfish, _) => "naturalist:entity/catfish",
            (Species::Clam, _) => "naturalist:entity/clam/brown_clam",
            (Species::Hippo, true) => "naturalist:entity/hippo/hippo_baby",
            (Species::Hippo, false) => "naturalist:entity/hippo/hippo",
            (Species::Piranha, _) => "naturalist:entity/piranha",
        }
    }
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

impl EntityRenderer for WildlifeRenderer {
    fn submit(
        &self,
        entity: &EntityRenderState,
        draw: &mut dyn BlockEntityDrawContext,
    ) -> Result<(), String> {
        let Some(living) = entity.living else {
            return Ok(());
        };
        let baby_model = living.is_baby && self.baby.is_some();
        let mut model = if baby_model {
            self.baby.as_ref().expect("baby model checked").clone()
        } else {
            self.adult.clone()
        };
        model.reset_all_poses();

        let phase = entity.age * 0.5;
        let stride = living.walk_animation_speed.clamp(0.0, 1.0);
        match self.species {
            Species::Alligator => {
                if let Some(tail) = bone(&mut model, "tail") {
                    tail.offset_rotation([0.0, phase.sin() * (0.08 + stride * 0.18), 0.0]);
                }
                if let Some(tail) = bone(&mut model, if baby_model { "tailTip" } else { "tail2" }) {
                    tail.offset_rotation([0.0, (phase - 0.5).sin() * (0.12 + stride * 0.24), 0.0]);
                }
                for (name, sign) in [("left_leg", 1.0), ("right_leg", -1.0)] {
                    if let Some(leg) = bone(&mut model, name) {
                        leg.offset_rotation([phase.sin() * stride * 0.25 * sign, 0.0, 0.0]);
                    }
                }
            }
            Species::Capybara | Species::Hippo => {
                for (name, sign) in [
                    ("leftLeg", 1.0),
                    ("rightLeg", -1.0),
                    ("left_leg", 1.0),
                    ("right_leg", -1.0),
                ] {
                    if let Some(leg) = bone(&mut model, name) {
                        leg.offset_rotation([phase.sin() * stride * 0.45 * sign, 0.0, 0.0]);
                    }
                }
                if let Some(skull) = bone(&mut model, "skull") {
                    skull.offset_rotation([living.x_rot.to_radians(), living.y_rot.to_radians(), 0.0]);
                }
            }
            Species::Catfish | Species::Piranha => {
                if let Some(tail) = bone(&mut model, "tail") {
                    tail.offset_rotation([0.0, phase.sin() * (0.12 + stride * 0.32), 0.0]);
                }
                if let Some(body) = bone(&mut model, "body") {
                    body.offset_rotation([living.x_rot.to_radians() * 0.35, 0.0, 0.0]);
                }
            }
            Species::Clam => {
                if let Some(top) = bone(&mut model, "top") {
                    top.offset_rotation([0.04 * phase.sin(), 0.0, 0.0]);
                }
            }
        }

        let age_scale = if baby_model { 1.0 } else { living.age_scale };
        draw.scale([living.scale * age_scale; 3]);
        draw.rotate_about([0.0, 1.0, 0.0], 180.0 - living.body_rot, [0.0; 3]);
        if living.death_time > 0.0 {
            let progress = (((living.death_time - 1.0) / 20.0 * 1.6).max(0.0))
                .sqrt()
                .min(1.0);
            draw.rotate_about([0.0, 0.0, 1.0], progress * 90.0, [0.0; 3]);
        }
        draw.scale([1.0, -1.0, -1.0]);
        draw.translate([0.0, -1.501, 0.0]);
        let tint = if living.has_red_overlay { 0xFF7070 } else { 0xFFFFFF };
        if !draw.model_part(&model, self.texture(baby_model), tint) {
            return Err(format!("missing Naturalist {:?} texture", self.species));
        }
        Ok(())
    }
}
