// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.5, 0.0, -3.5], [3.0, 0.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -4.1, -1.5));
    let rightWing = mesh.add_or_replace_child(body, "rightWing", CubeListBuilder::create().tex_offs(0, 14).add_box_with([-7.0, 0.0, -7.0], [7.0, 0.0, 13.0], CubeDeformation::new(0.0)), PartPose::offset(-0.5, 0.0, 1.5));
    let leftWing = mesh.add_or_replace_child(body, "leftWing", CubeListBuilder::create().tex_offs(0, 27).set_mirror(true).add_box_with([0.0, 0.0, -7.0], [7.0, 0.0, 13.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.5, 0.0, 1.5));
    LayerDefinition::create(mesh, 32, 64).bake_root()
}
