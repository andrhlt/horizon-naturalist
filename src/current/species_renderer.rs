//! Java model geometry for the reptiles and burrowing trail entities.
use mod_api::{BlockEntityDrawContext, EntityRenderState, EntityRenderer, ModelPart};
use std::f32::consts::PI;

#[path = "species_models/komodo_dragon.rs"] mod komodo;
#[path = "species_models/lizard.rs"] mod lizard;
#[path = "species_models/lizard_tail.rs"] mod lizard_tail;
#[path = "species_models/dirt_trail.rs"] mod dirt_trail;
#[path = "species_models/tortoise.rs"] mod tortoise;
#[path = "species_models/mole.rs"] mod mole;

#[derive(Clone, Copy)]
pub(crate) enum Species { Komodo, Lizard, Tail, Trail, Tortoise, Mole }

pub(crate) struct SpeciesRenderer {
    species: Species,
    adult: ModelPart,
    baby: Option<ModelPart>,
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

impl SpeciesRenderer {
    pub fn new(species: Species) -> Self {
        let (adult, baby) = match species {
            Species::Komodo => (komodo::adult(), None),
            Species::Lizard => (lizard::adult(), None),
            Species::Tail => (lizard_tail::adult(), None),
            Species::Trail => (dirt_trail::adult(), None),
            Species::Tortoise => (tortoise::adult(), Some(tortoise::baby())),
            Species::Mole => (mole::adult(), None),
        };
        Self { species, adult, baby }
    }
}

impl EntityRenderer for SpeciesRenderer {
    fn submit(&self, entity: &EntityRenderState, draw: &mut dyn BlockEntityDrawContext) -> Result<(), String> {
        if matches!(self.species, Species::Trail) {
            draw.rotate_about([0.0, 1.0, 0.0], 180.0 - entity.yaw, [0.0; 3]);
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            if !draw.model_part(&self.adult, "naturalist:entity/mole", 0xFFFFFF) {
                return Err("missing Naturalist texture naturalist:entity/mole".into());
            }
            return Ok(());
        }
        let living = entity.living.ok_or_else(|| "Naturalist species renderer requires living state".to_owned())?;
        let is_baby = living.is_baby && self.baby.is_some();
        let mut model = if is_baby { self.baby.as_ref().expect("baby model").clone() } else { self.adult.clone() };
        model.reset_all_poses();
        let stride = (living.walk_animation_pos * PI).sin() * living.walk_animation_speed * 0.55;
        for (name, angle) in [
            ("left_arm", stride),
            ("right_arm", -stride),
            ("left_leg", -stride),
            ("right_leg", stride),
        ] {
            if let Some(limb) = bone(&mut model, name) {
                limb.offset_rotation([angle, 0.0, 0.0]);
            }
        }
        if matches!(self.species, Species::Tail)
            && let Some(root) = bone(&mut model, "root")
        {
            root.offset_rotation([0.0, (entity.age * 0.2).sin() * 0.3, 0.0]);
        }
        let texture = match self.species {
            Species::Komodo => "naturalist:entity/komodo_dragon",
            Species::Lizard => "naturalist:entity/lizard/green",
            Species::Tail => "naturalist:entity/lizard/tail",
            Species::Trail => "naturalist:entity/mole",
            Species::Tortoise => if is_baby { "naturalist:entity/tortoise/brown_baby" } else { "naturalist:entity/tortoise/brown" },
            Species::Mole => "naturalist:entity/mole",
        };
        draw.scale([living.scale; 3]);
        draw.rotate_about([0.0, 1.0, 0.0], 180.0 - living.body_rot, [0.0; 3]);
        draw.scale([1.0, -1.0, -1.0]);
        draw.translate([0.0, -1.501, 0.0]);
        let tint = if living.has_red_overlay { 0xFF7070 } else { 0xFFFFFF };
        if !draw.model_part(&model, texture, tint) { return Err(format!("missing Naturalist texture {texture}")); }
        Ok(())
    }
}
