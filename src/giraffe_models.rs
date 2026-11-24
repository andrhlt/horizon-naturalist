// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 26.0, 0.0));
    let hips = mesh.add_or_replace_child(root, "hips", CubeListBuilder::create(), PartPose::offset(0.0, -33.6174, 12.6415));
    let shoulders = mesh.add_or_replace_child(hips, "shoulders", CubeListBuilder::create(), PartPose::offset(0.0, 1.0, -23.0));
    let body = mesh.add_or_replace_child(shoulders, "body", CubeListBuilder::create().tex_offs(1, 0).add_box_with([-7.5, -14.3826, -14.6415], [15.0, 20.0, 28.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -6.0, 10.0));
    let seat = mesh.add_or_replace_child(body, "seat", CubeListBuilder::create(), PartPose::offset(0.0, -12.3826, -3.6415));
    let neck = mesh.add_or_replace_child(body, "neck", CubeListBuilder::create(), PartPose::offset(0.0, -7.2347, -3.717));
    let neck_r1 = mesh.add_or_replace_child(neck, "neck_r1", CubeListBuilder::create().tex_offs(1, 49).add_box_with([-4.5, -54.8521, -14.0755], [9.0, 17.0, 20.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 38.9545, 20.5467, 0.3927, 0.0, 0.0));
    let head = mesh.add_or_replace_child(neck, "head", CubeListBuilder::create().tex_offs(17, 15).add_box_with([1.0, -37.5573, -12.5922], [2.0, 5.0, 2.0], CubeDeformation::new(0.0)).tex_offs(17, 8).add_box_with([1.0, -37.5573, -12.5922], [2.0, 5.0, 2.0], CubeDeformation::new(0.25)).tex_offs(17, 15).set_mirror(true).add_box_with([-3.0, -37.5573, -12.5922], [2.0, 5.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(17, 8).set_mirror(true).add_box_with([-3.0, -37.5573, -12.5922], [2.0, 5.0, 2.0], CubeDeformation::new(0.25)).set_mirror(false), PartPose::offset(0.0, -4.8079, -9.2977));
    let head_r1 = mesh.add_or_replace_child(head, "head_r1", CubeListBuilder::create().tex_offs(0, 4).add_box_with([3.5, -86.8521, -6.8255], [5.0, 3.0, 1.0], CubeDeformation::new(0.0)).tex_offs(0, 0).add_box_with([-8.5, -86.8521, -6.8255], [5.0, 3.0, 1.0], CubeDeformation::new(0.0)).tex_offs(36, 90).add_box_with([-1.5, -86.8521, -4.8255], [3.0, 33.0, 4.0], CubeDeformation::new(0.0)).tex_offs(62, 49).add_box_with([-3.5, -86.8521, -13.8255], [7.0, 33.0, 9.0], CubeDeformation::new(0.0)).tex_offs(31, 52).add_box_with([-1.5, -86.8521, -16.8255], [3.0, 2.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 43.7624, 29.8444, 0.3927, 0.0, 0.0));
    let snout = mesh.add_or_replace_child(head, "snout", CubeListBuilder::create().tex_offs(0, 16).add_box_with([-2.5, -4.0, -7.0], [5.0, 4.0, 7.0], CubeDeformation::new(0.0)).tex_offs(63, 16).add_box_with([-2.5, -4.0, -7.0], [5.0, 4.0, 7.0], CubeDeformation::new(0.25)), PartPose::offset_and_rotation(0.0, -25.6445, -13.8695, 0.3927, 0.0, 0.0));
    let saddle = mesh.add_or_replace_child(neck, "saddle", CubeListBuilder::create().tex_offs(69, 91).add_box_with([-4.5, -54.8521, -14.0755], [9.0, 17.0, 20.0], CubeDeformation::new(0.5)), PartPose::offset_and_rotation(0.0, 38.9545, 20.5467, 0.3927, 0.0, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create(), PartPose::offset(0.0, -9.3826, 13.3585));
    let tail_r1 = mesh.add_or_replace_child(tail, "tail_r1", CubeListBuilder::create().tex_offs(58, 0).add_box_with([-2.5, 0.0, -32.0], [5.0, 0.0, 32.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 1.9635, 0.0, 0.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(6.0, -34.0, 12.5));
    let back_legs = mesh.add_or_replace_child(legs, "back_legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let leftFoot = mesh.add_or_replace_child(back_legs, "leftFoot", CubeListBuilder::create(), PartPose::offset(-1.0, 32.0, -2.5));
    let left_leg = mesh.add_or_replace_child(leftFoot, "left_leg", CubeListBuilder::create().tex_offs(0, 90).add_box_with([-2.0, 0.0, -2.5], [4.0, 32.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -32.0, 2.5));
    let rightFoot = mesh.add_or_replace_child(back_legs, "rightFoot", CubeListBuilder::create(), PartPose::offset(-11.0, 32.0, -2.5));
    let right_leg = mesh.add_or_replace_child(rightFoot, "right_leg", CubeListBuilder::create().tex_offs(0, 90).add_box_with([-2.0, 0.0, -2.5], [4.0, 32.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -32.0, 2.5));
    let front_legs = mesh.add_or_replace_child(legs, "front_legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, -23.0));
    let leftHand = mesh.add_or_replace_child(front_legs, "leftHand", CubeListBuilder::create(), PartPose::offset(-1.0, 32.0, -2.5));
    let left_arm = mesh.add_or_replace_child(leftHand, "left_arm", CubeListBuilder::create().tex_offs(0, 90).add_box_with([-2.0, 0.0, -2.5], [4.0, 32.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -32.0, 2.5));
    let rightHand = mesh.add_or_replace_child(front_legs, "rightHand", CubeListBuilder::create(), PartPose::offset(-11.0, 32.0, -2.5));
    let right_arm = mesh.add_or_replace_child(rightHand, "right_arm", CubeListBuilder::create().tex_offs(0, 90).set_mirror(true).add_box_with([-2.0, 0.0, -2.5], [4.0, 32.0, 5.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, -32.0, 2.5));
    LayerDefinition::create(mesh, 128, 128).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 7.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-3.5, -4.0, -6.5], [7.0, 8.0, 13.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.0, 0.5));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create(), PartPose::offset(0.0, -4.0, 6.5));
    let tail_r1 = mesh.add_or_replace_child(tail, "tail_r1", CubeListBuilder::create().tex_offs(-13, 41).add_box_with([-3.5, -22.0, 7.0], [7.0, 0.0, 13.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -2.0724, -22.9936, -1.3526, 0.0, 0.0));
    let neck = mesh.add_or_replace_child(root, "neck", CubeListBuilder::create().tex_offs(0, 21).add_box_with([-1.5, -13.0, -3.5], [3.0, 15.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -3.0, -4.5));
    let bone = mesh.add_or_replace_child(neck, "bone", CubeListBuilder::create().tex_offs(16, 32).add_box_with([-0.5, -37.0, -3.0], [1.0, 15.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 20.0, 4.5));
    let skull2 = mesh.add_or_replace_child(neck, "skull2", CubeListBuilder::create().tex_offs(36, 32).add_box_with([-2.0, -3.0, -9.5], [4.0, 3.0, 5.0], CubeDeformation::new(0.0)).tex_offs(40, 0).add_box_with([-2.0, -3.0, -9.5], [4.0, 3.0, 5.0], CubeDeformation::new(0.25)).tex_offs(16, 21).add_box_with([-3.0, -5.0, -4.5], [6.0, 5.0, 6.0], CubeDeformation::new(0.025)).tex_offs(40, 11).add_box_with([-3.0, -8.0, -0.5], [2.0, 3.0, 2.0], CubeDeformation::new(0.0)).tex_offs(40, 16).add_box_with([-3.0, -8.0, -0.5], [2.0, 3.0, 2.0], CubeDeformation::new(0.25)).tex_offs(40, 11).set_mirror(true).add_box_with([1.0, -8.0, -0.5], [2.0, 3.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(40, 16).set_mirror(true).add_box_with([1.0, -8.0, -0.5], [2.0, 3.0, 2.0], CubeDeformation::new(0.25)).set_mirror(false), PartPose::offset(0.0, -12.0, 0.0));
    let rightEar = mesh.add_or_replace_child(skull2, "rightEar", CubeListBuilder::create().tex_offs(40, 8).add_box_with([-4.0, -1.0, -0.5], [4.0, 2.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(-3.0, -4.0, 0.0));
    let leftEar = mesh.add_or_replace_child(skull2, "leftEar", CubeListBuilder::create().tex_offs(40, 8).set_mirror(true).add_box_with([0.5, -1.0, -0.5], [4.0, 2.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(2.5, -4.0, 0.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(-1.5, 2.0, 0.5));
    let backLegs = mesh.add_or_replace_child(legs, "backLegs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 5.0));
    let rightLeg = mesh.add_or_replace_child(backLegs, "rightLeg", CubeListBuilder::create().tex_offs(26, 32).add_box_with([-1.5, 1.0, -1.5], [2.0, 14.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let leftLeg = mesh.add_or_replace_child(backLegs, "leftLeg", CubeListBuilder::create().tex_offs(26, 32).set_mirror(true).add_box_with([-0.5, 1.0, -1.5], [2.0, 14.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(3.0, 0.0, 0.0));
    let frontLegs = mesh.add_or_replace_child(legs, "frontLegs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, -5.0));
    let rightArm = mesh.add_or_replace_child(frontLegs, "rightArm", CubeListBuilder::create().tex_offs(26, 32).add_box_with([-1.5, 1.0, -1.5], [2.0, 14.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let leftArm = mesh.add_or_replace_child(frontLegs, "leftArm", CubeListBuilder::create().tex_offs(26, 32).set_mirror(true).add_box_with([-0.5, 1.0, -1.5], [2.0, 14.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(3.0, 0.0, 0.0));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
