// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 23.0, 1.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let front = mesh.add_or_replace_child(body, "front", CubeListBuilder::create().tex_offs(0, 14).add_box_with([-1.5, -2.0, -7.0], [3.0, 3.0, 7.0], CubeDeformation::new(0.0)).tex_offs(23, 8).add_box_with([-2.0, -2.0, -7.25], [4.0, 3.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let eyes = mesh.add_or_replace_child(front, "eyes", CubeListBuilder::create(), PartPose::offset(0.0, -2.0, -7.0));
    let rightEye = mesh.add_or_replace_child(eyes, "rightEye", CubeListBuilder::create().tex_offs(20, 21).add_box_with([-1.5, -4.0, 0.0], [2.0, 4.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(-1.0, 0.0, 0.0));
    let leftEye = mesh.add_or_replace_child(eyes, "leftEye", CubeListBuilder::create().tex_offs(20, 21).set_mirror(true).add_box_with([-0.5, -4.0, 0.0], [2.0, 4.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(1.0, 0.0, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(20, 14).add_box_with([-1.5, -1.0, 0.0], [3.0, 2.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let shell = mesh.add_or_replace_child(root, "shell", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-2.0, -3.5, -3.5], [4.0, 7.0, 7.0], CubeDeformation::new(0.0)).tex_offs(0, 24).add_box_with([-2.0, -3.5, -3.5], [4.0, 7.0, 7.0], CubeDeformation::new(0.5)), PartPose::offset(0.0, -4.5, -0.5));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
