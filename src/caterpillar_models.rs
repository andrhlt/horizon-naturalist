// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create(), PartPose::offset(0.0, -1.5, -3.0));
    let neck = mesh.add_or_replace_child(body, "neck", CubeListBuilder::create().tex_offs(20, 2).add_box_with([-1.5, -1.5, -2.0], [3.0, 3.0, 2.0], CubeDeformation::new(0.03)), PartPose::offset(0.0, 0.0, -1.0));
    let antennae = mesh.add_or_replace_child(neck, "antennae", CubeListBuilder::create().tex_offs(0, 24).add_box_with([-1.5, -2.0, -3.0], [3.0, 2.0, 3.0], CubeDeformation::new(0.03)), PartPose::offset(0.0, -1.5, -2.0));
    let extended = mesh.add_or_replace_child(body, "extended", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 2.5));
    let chest = mesh.add_or_replace_child(extended, "chest", CubeListBuilder::create().tex_offs(7, 7).add_box_with([-1.5, -1.5, -5.0], [3.0, 3.0, 5.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, 0.0, 1.5));
    let butt = mesh.add_or_replace_child(extended, "butt", CubeListBuilder::create().tex_offs(41, 23).add_box_with([-1.5, -1.5, 0.0], [3.0, 3.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 1.5));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
