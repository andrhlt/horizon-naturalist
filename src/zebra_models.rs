// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 32).add_box_with([-5.0, -5.0, -11.0], [10.0, 10.0, 22.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -16.0, 0.0));
    let neck = mesh.add_or_replace_child(body, "neck", CubeListBuilder::create(), PartPose::offset(0.0, -0.75, -8.0));
    let neck_r1 = mesh.add_or_replace_child(neck, "neck_r1", CubeListBuilder::create().tex_offs(0, 35).add_box_with([-2.0, -27.75, -11.0], [4.0, 12.0, 7.0], CubeDeformation::new(0.0)).tex_offs(56, 36).add_box_with([-1.0, -32.75, -4.0], [2.0, 16.0, 2.0], CubeDeformation::new(0.0)).tex_offs(0, 25).add_box_with([-2.0, -32.75, -16.0], [4.0, 5.0, 5.0], CubeDeformation::new(0.0)).tex_offs(0, 13).add_box_with([-3.0, -32.75, -11.0], [6.0, 5.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 10.5059, 15.3032, 0.5236, 0.0, 0.0));
    let leftEar = mesh.add_or_replace_child(neck, "leftEar", CubeListBuilder::create(), PartPose::offset(1.5975, -15.1003, -4.6882));
    let leftEar_r1 = mesh.add_or_replace_child(leftEar, "leftEar_r1", CubeListBuilder::create().tex_offs(19, 14).set_mirror(true).add_box_with([1.0458, -36.862, -5.01], [2.0, 5.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-9.8696, 24.2988, 19.2366, 0.5087, -0.1298, 0.228));
    let rightEar = mesh.add_or_replace_child(neck, "rightEar", CubeListBuilder::create(), PartPose::offset(-1.5975, -15.1003, -4.6882));
    let rightEar_r1 = mesh.add_or_replace_child(rightEar, "rightEar_r1", CubeListBuilder::create().tex_offs(19, 14).add_box_with([-3.0458, -36.862, -5.01], [2.0, 5.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(9.8696, 24.2988, 19.2366, 0.5087, 0.1298, -0.228));
    let bridle = mesh.add_or_replace_child(neck, "bridle", CubeListBuilder::create(), PartPose::offset(0.0, 10.7224, 15.4282));
    let bridle_r1 = mesh.add_or_replace_child(bridle, "bridle_r1", CubeListBuilder::create().tex_offs(19, 0).add_box_with([-2.0, -32.75, -12.0], [4.0, 5.0, 2.0], CubeDeformation::new(0.05)).tex_offs(29, 5).add_box_with([2.0, -30.75, -13.0], [1.0, 2.0, 2.0], CubeDeformation::new(0.0)).tex_offs(29, 5).set_mirror(true).add_box_with([-3.0, -30.75, -13.0], [1.0, 2.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(1, 1).add_box_with([-3.0, -32.75, -11.0], [6.0, 5.0, 6.0], CubeDeformation::new(0.05)), PartPose::offset_and_rotation(0.0, -0.2165, -0.125, 0.5236, 0.0, 0.0));
    let reinsR = mesh.add_or_replace_child(neck, "reinsR", CubeListBuilder::create(), PartPose::offset(-3.1, -9.4413, -10.6471));
    let reinsR_r1 = mesh.add_or_replace_child(reinsR, "reinsR_r1", CubeListBuilder::create().tex_offs(32, 2).set_mirror(true).add_box_with([-3.1, -31.75, -12.0], [0.0, 3.0, 16.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(3.1, 31.2243, 9.816, -0.0873, 0.0, 0.0));
    let reinsL = mesh.add_or_replace_child(neck, "reinsL", CubeListBuilder::create(), PartPose::offset(3.1, -9.4413, -10.6471));
    let reinsL_r1 = mesh.add_or_replace_child(reinsL, "reinsL_r1", CubeListBuilder::create().tex_offs(32, 2).add_box_with([3.1, -31.75, -12.0], [0.0, 3.0, 16.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-3.1, 31.2243, 9.816, -0.0873, 0.0, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create(), PartPose::offset(0.0, -4.0, 11.0));
    let tail_r1 = mesh.add_or_replace_child(tail, "tail_r1", CubeListBuilder::create().tex_offs(47, 39).add_box_with([-0.5, -20.0, 10.5], [1.0, 7.0, 1.0], CubeDeformation::new(0.0)).tex_offs(45, 47).add_box_with([-1.0, -13.0, 10.0], [2.0, 5.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 22.8205, 0.4737, 0.5236, 0.0, 0.0));
    let chest = mesh.add_or_replace_child(body, "chest", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let chest_left = mesh.add_or_replace_child(chest, "chest_left", CubeListBuilder::create().tex_offs(26, 21).set_mirror(true).add_box_with([-7.0, 7.0, 0.0], [8.0, 8.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(5.0, -12.0, 3.0, 0.0, 1.5708, 0.0));
    let chest_right = mesh.add_or_replace_child(chest, "chest_right", CubeListBuilder::create().tex_offs(26, 21).set_mirror(true).add_box_with([-8.0, 7.0, 0.0], [8.0, 8.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-5.0, -12.0, 10.0, 0.0, -1.5708, 0.0));
    let saddle = mesh.add_or_replace_child(body, "saddle", CubeListBuilder::create().tex_offs(26, 0).set_mirror(true).add_box_with([-5.0, -4.5, -4.0], [10.0, 9.0, 9.0], CubeDeformation::new(0.5)).set_mirror(false), PartPose::offset(0.0, -0.5, 0.5));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(0.0, -12.0, 0.0));
    let frontLegs = mesh.add_or_replace_child(legs, "frontLegs", CubeListBuilder::create(), PartPose::offset(0.0, 12.0, 0.0));
    let rightArm = mesh.add_or_replace_child(frontLegs, "rightArm", CubeListBuilder::create().tex_offs(48, 21).add_box_with([-2.0, 0.0, -2.0], [4.0, 11.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(-2.9, -11.0, -9.0));
    let leftArm = mesh.add_or_replace_child(frontLegs, "leftArm", CubeListBuilder::create().tex_offs(48, 21).set_mirror(true).add_box_with([-2.0, 0.0, -2.0], [4.0, 11.0, 4.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(2.9, -11.0, -9.0));
    let backLegs = mesh.add_or_replace_child(legs, "backLegs", CubeListBuilder::create(), PartPose::offset(0.0, 12.0, 0.0));
    let leftLeg = mesh.add_or_replace_child(backLegs, "leftLeg", CubeListBuilder::create().tex_offs(48, 21).set_mirror(true).add_box_with([-2.0, 0.0, -2.0], [4.0, 11.0, 4.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(2.9, -11.0, 9.0));
    let rightLeg = mesh.add_or_replace_child(backLegs, "rightLeg", CubeListBuilder::create().tex_offs(48, 21).add_box_with([-2.0, 0.0, -2.0], [4.0, 11.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(-2.9, -11.0, 9.0));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(-0.375, -8.5, 0.0));
    let rightArm = mesh.add_or_replace_child(legs, "rightArm", CubeListBuilder::create().tex_offs(26, 33).add_box_with([0.25, -0.5, -1.5], [2.0, 9.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(-2.875, 0.0, -5.5));
    let leftArm = mesh.add_or_replace_child(legs, "leftArm", CubeListBuilder::create().tex_offs(26, 33).set_mirror(true).add_box_with([-0.75, -0.5, -1.5], [2.0, 9.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(2.125, 0.0, -5.5));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create().tex_offs(26, 33).add_box_with([-0.5, -0.5, -1.5], [2.0, 9.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(-2.125, 0.0, 5.5));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create().tex_offs(26, 33).add_box_with([-1.5, -0.5, -1.5], [2.0, 9.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(2.875, 0.0, 5.5));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-3.0, -3.5, -7.0], [6.0, 7.0, 14.0], CubeDeformation::new(0.026)), PartPose::offset(0.0, -12.5, 0.0));
    let neck = mesh.add_or_replace_child(body, "neck", CubeListBuilder::create().tex_offs(26, 21).add_box_with([-1.25, -6.2078, -1.4434], [4.0, 8.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-0.75, -1.984, -6.3572, 0.3927, 0.0, 0.0));
    let skull2 = mesh.add_or_replace_child(neck, "skull2", CubeListBuilder::create().tex_offs(0, 21).add_box_with([-2.0, -3.6918, -6.8006], [4.0, 4.0, 9.0], CubeDeformation::new(0.0)), PartPose::offset(0.75, -6.516, 0.3572));
    let leftEar = mesh.add_or_replace_child(skull2, "leftEar", CubeListBuilder::create().tex_offs(12, 34).set_mirror(true).add_box_with([-1.0, -4.1918, -0.8006], [2.0, 4.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(2.0, -3.5, 2.0));
    let rightEar = mesh.add_or_replace_child(skull2, "rightEar", CubeListBuilder::create().tex_offs(12, 34).add_box_with([-1.0, -4.1918, -0.8006], [2.0, 4.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(-2.0, -3.5, 2.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(8, 34).add_box_with([-0.5, 0.0, -0.5], [1.0, 7.0, 1.0], CubeDeformation::new(0.0)).tex_offs(0, 34).add_box_with([-1.0, 7.0, -1.0], [2.0, 5.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -2.5, 7.0, 0.5236, 0.0, 0.0));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
