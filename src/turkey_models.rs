// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 22.0, 0.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(-2.0, -3.25, 1.0));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create().tex_offs(12, 9).set_mirror(true).add_box_with([-1.5, 0.25, -3.0], [3.0, 2.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(24, 6).set_mirror(true).add_box_with([-1.5, 0.25, -3.0], [3.0, 5.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(4.0, 0.0, 0.0));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create().tex_offs(12, 9).add_box_with([-1.5, 0.25, -3.0], [3.0, 2.0, 3.0], CubeDeformation::new(0.0)).tex_offs(24, 6).add_box_with([-1.5, 0.25, -3.0], [3.0, 5.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 14).add_box_with([-4.5, -5.0, -6.0], [9.0, 10.0, 12.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -8.0, -1.0));
    let skull = mesh.add_or_replace_child(body, "skull", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.5, -8.0, -2.0], [3.0, 11.0, 3.0], CubeDeformation::new(0.0)).tex_offs(0, 14).add_box_with([-0.5, -7.0, -5.0], [1.0, 3.0, 3.0], CubeDeformation::new(0.0)).tex_offs(12, 0).add_box_with([-1.0, -6.0, -4.0], [2.0, 4.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -3.0, -6.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(0, 36).add_box_with([-5.5, -4.0, -1.0], [11.0, 6.0, 2.0], CubeDeformation::new(0.0)).tex_offs(0, 47).add_box_with([-12.5, -13.0, 1.1], [25.0, 17.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -2.75, 6.0, -0.3927, 0.0, 0.0));
    let leftWing = mesh.add_or_replace_child(body, "leftWing", CubeListBuilder::create().tex_offs(43, 19).set_mirror(true).add_box_with([-0.5, -0.5, -3.5], [1.0, 9.0, 7.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(5.0, 0.5, -1.0, 0.7854, 0.0, 0.0));
    let rightWing = mesh.add_or_replace_child(body, "rightWing", CubeListBuilder::create().tex_offs(43, 19).add_box_with([-0.5, -0.5, -3.5], [1.0, 9.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-5.0, 0.5, -1.0, 0.7854, 0.0, 0.0));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
