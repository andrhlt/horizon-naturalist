// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.5, -1.5, -5.0], [3.0, 3.0, 10.0], CubeDeformation::new(0.0)).tex_offs(0, 3).add_box_with([0.0, -5.5, -5.0], [0.0, 4.0, 10.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 22.5, 0.0));
    LayerDefinition::create(mesh, 32, 32).bake_root()
}
