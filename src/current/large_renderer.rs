//! Current Horizon renderers for the seven grassland/large animals.
//!
//! The meshes remain the Java-derived Naturalist models; this adapter keeps the
//! legacy geometry while using Horizon's current `EntityRenderer` surface.
use mod_api::{BlockEntityDrawContext, EntityRenderState, EntityRenderer, ModelPart};

#[path = "../elephant_models.rs"] mod elephant_models;
#[path = "../mammoth_models.rs"] mod mammoth_models;
#[path = "../giraffe_models.rs"] mod giraffe_models;
#[path = "../lion_models.rs"] mod lion_models;
#[path = "../rhino_models.rs"] mod rhino_models;
#[path = "../tiger_models.rs"] mod tiger_models;
#[path = "../zebra_models.rs"] mod zebra_models;

#[derive(Clone, Copy)]
enum Kind { Elephant, Mammoth, Giraffe, Lion, Rhino, Tiger, Zebra }

pub(crate) struct LargeRenderer { kind: Kind, adult: ModelPart, baby: ModelPart }

impl LargeRenderer {
    fn new(kind: Kind, adult: ModelPart, baby: ModelPart) -> Self { Self { kind, adult, baby } }
    pub(crate) fn elephant() -> Self { Self::new(Kind::Elephant, elephant_models::adult(), elephant_models::baby()) }
    pub(crate) fn mammoth() -> Self { Self::new(Kind::Mammoth, mammoth_models::adult(), mammoth_models::baby()) }
    pub(crate) fn giraffe() -> Self { Self::new(Kind::Giraffe, giraffe_models::adult(), giraffe_models::baby()) }
    pub(crate) fn lion() -> Self { Self::new(Kind::Lion, lion_models::adult(), lion_models::baby()) }
    pub(crate) fn rhino() -> Self { Self::new(Kind::Rhino, rhino_models::adult(), rhino_models::adult()) }
    pub(crate) fn tiger() -> Self { Self::new(Kind::Tiger, tiger_models::adult(), tiger_models::baby()) }
    pub(crate) fn zebra() -> Self { Self::new(Kind::Zebra, zebra_models::adult(), zebra_models::baby()) }
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
}

impl EntityRenderer for LargeRenderer {
    fn submit(&self, entity: &EntityRenderState, draw: &mut dyn BlockEntityDrawContext) -> Result<(), String> {
        let Some(living) = entity.living else { return Ok(()) };
        let model = if living.is_baby { self.baby.clone() } else { self.adult.clone() };
        draw.rotate_about([0.0, 1.0, 0.0], entity.yaw, [0.0; 3]);
        draw.rotate_about([1.0, 0.0, 0.0], entity.pitch, [0.0; 3]);
        draw.scale([living.scale, -living.scale, -living.scale]);
        draw.translate([0.0, -1.501 * living.age_scale, 0.0]);
        let tint = if living.has_red_overlay { 0xFF7070 } else { 0xFFFFFF };
        if !draw.model_part(&model, self.texture(living.is_baby), tint) {
            return Err(format!("missing Naturalist texture for {:?}", self.kind as u8));
        }
        Ok(())
    }
}
