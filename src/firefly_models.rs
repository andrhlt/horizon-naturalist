// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.5, -1.5, -2.5], [3.0, 3.0, 5.0], CubeDeformation::new(0.0)).tex_offs(12, 2).add_box_with([-1.5, -2.5, -3.5], [3.0, 1.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -2.5, 0.0));
    let glow = mesh.add_or_replace_child(body, "glow", CubeListBuilder::create().tex_offs(0, 24).add_box_with([-1.5, -4.0, -2.5], [3.0, 3.0, 5.0], CubeDeformation::new(0.3)), PartPose::offset(0.0, 2.5, 0.0));
    let rightWing = mesh.add_or_replace_child(body, "rightWing", CubeListBuilder::create().tex_offs(-4, 8).add_box_with([-5.0, 0.0, -1.0], [5.0, 0.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-0.5, -1.75, -0.5, 0.0, 0.0, 1.0472));
    let leftWing = mesh.add_or_replace_child(body, "leftWing", CubeListBuilder::create().tex_offs(-4, 8).set_mirror(true).add_box_with([0.0, 0.0, -1.0], [5.0, 0.0, 4.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(0.5, -1.75, -0.5, 0.0, 0.0, -1.0472));
    LayerDefinition::create(mesh, 32, 32).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create(), PartPose::offset(0.0, -1.0, 2.0));
    let butt = mesh.add_or_replace_child(body, "butt", CubeListBuilder::create().tex_offs(2, 0).set_mirror(true).add_box_with([-1.0, -1.0, 0.0], [2.0, 2.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 0.0, -2.0));
    let neck = mesh.add_or_replace_child(body, "neck", CubeListBuilder::create().tex_offs(0, 4).add_box_with([-1.0, -1.0, -2.0], [2.0, 2.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, -2.0));
    let antenna = mesh.add_or_replace_child(neck, "antenna", CubeListBuilder::create().tex_offs(0, 10).add_box_with([-1.0, -1.0, -1.0], [2.0, 1.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.0, -2.0));
    LayerDefinition::create(mesh, 16, 16).bake_root()
}
