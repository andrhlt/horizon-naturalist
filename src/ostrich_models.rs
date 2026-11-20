// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, -2.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-5.0, -6.5, -8.0], [10.0, 13.0, 16.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -21.5, 2.0));
    let saddle = mesh.add_or_replace_child(body, "saddle", CubeListBuilder::create().tex_offs(63, 0).add_box_with([-5.0, -28.0, -3.0], [10.0, 9.0, 9.0], CubeDeformation::new(0.5)), PartPose::offset(0.0, 21.5, -4.0));
    let skull = mesh.add_or_replace_child(body, "skull", CubeListBuilder::create().tex_offs(1, 75).add_box_with([-0.5, -17.0, -3.0], [3.0, 20.0, 3.0], CubeDeformation::new(0.0)).tex_offs(14, 74).add_box_with([-0.5, -16.0, -7.0], [3.0, 2.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(-1.0, -2.5, -8.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(2, 32).add_box_with([-3.0, -4.0, -1.0], [6.0, 8.0, 8.0], CubeDeformation::new(0.0)).tex_offs(17, 49).add_box_with([-3.0, -4.0, 7.0], [6.0, 8.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -4.5, 5.0));
    let leftWing = mesh.add_or_replace_child(body, "leftWing", CubeListBuilder::create().tex_offs(35, 32).add_box_with([-1.0, 0.0, -1.0], [2.0, 8.0, 10.0], CubeDeformation::new(0.0)).tex_offs(60, 39).add_box_with([-1.0, 0.0, 9.0], [2.0, 8.0, 3.0], CubeDeformation::new(0.01)), PartPose::offset(6.0, -0.5, -2.0));
    let rightWing = mesh.add_or_replace_child(body, "rightWing", CubeListBuilder::create().tex_offs(35, 32).set_mirror(true).add_box_with([-1.0, 0.0, -1.0], [2.0, 8.0, 10.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(60, 39).set_mirror(true).add_box_with([-1.0, 0.0, 9.0], [2.0, 8.0, 3.0], CubeDeformation::new(0.01)).set_mirror(false), PartPose::offset(-6.0, -0.5, -2.0));
    let seat = mesh.add_or_replace_child(body, "seat", CubeListBuilder::create(), PartPose::offset(0.0, -6.0, 0.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(2.75, -14.5, 3.0));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create().tex_offs(54, 20).set_mirror(true).add_box_with([-1.0, -0.5, -1.0], [2.0, 15.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 0.0, 0.0));
    let leftFoot = mesh.add_or_replace_child(leftLeg, "leftFoot", CubeListBuilder::create().tex_offs(64, 20).set_mirror(true).add_box_with([-2.0, -0.5, -4.5], [4.0, 1.0, 6.0], CubeDeformation::new(0.02)).set_mirror(false), PartPose::offset(0.0, 14.0, 0.5));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create().tex_offs(54, 20).add_box_with([-1.0, -0.5, -1.0], [2.0, 15.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(-5.5, 0.0, 0.0));
    let rightFoot = mesh.add_or_replace_child(rightLeg, "rightFoot", CubeListBuilder::create().tex_offs(64, 20).add_box_with([-2.0, -0.5, -4.5], [4.0, 1.0, 6.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, 14.0, 0.5));
    LayerDefinition::create(mesh, 128, 128).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 12).add_box_with([-2.5, -3.0, -3.0], [5.0, 6.0, 6.0], CubeDeformation::new(0.25)).tex_offs(0, 0).add_box_with([-2.5, -3.0, -3.0], [5.0, 6.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -7.0, 0.0));
    let leftWing = mesh.add_or_replace_child(body, "leftWing", CubeListBuilder::create().tex_offs(22, 23).add_box_with([0.0, 0.0, -1.5], [2.0, 0.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(2.5, -1.0, -1.5));
    let rightWing = mesh.add_or_replace_child(body, "rightWing", CubeListBuilder::create().tex_offs(22, 23).set_mirror(true).add_box_with([-2.0, 0.0, -1.5], [2.0, 0.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-2.5, -1.0, -1.5));
    let skull = mesh.add_or_replace_child(body, "skull", CubeListBuilder::create().tex_offs(22, 0).add_box_with([-1.0, -7.0, -2.0], [2.0, 8.0, 2.0], CubeDeformation::new(0.0)).tex_offs(22, 10).add_box_with([-1.0, -7.0, -2.0], [2.0, 8.0, 2.0], CubeDeformation::new(0.25)).tex_offs(0, 24).add_box_with([-1.0, -6.0, -4.0], [2.0, 1.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.0, -3.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(1.5, -4.0, 1.0));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create().tex_offs(8, 24).add_box_with([-0.5, 0.0, 0.0], [1.0, 4.0, 0.0], CubeDeformation::new(0.0)).tex_offs(22, 20).add_box_with([-1.5, 4.0, -3.0], [3.0, 0.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create().tex_offs(8, 24).set_mirror(true).add_box_with([-0.5, 0.0, 0.0], [1.0, 4.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(22, 20).set_mirror(true).add_box_with([-1.5, 4.0, -3.0], [3.0, 0.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-3.0, 0.0, 0.0));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
