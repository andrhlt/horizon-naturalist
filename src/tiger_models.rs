// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-5.0, -6.0, -13.0], [10.0, 12.0, 26.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -16.6, -1.5));
    let skullRot = mesh.add_or_replace_child(body, "skullRot", CubeListBuilder::create(), PartPose::offset(0.0, -3.9, -12.875));
    let skull = mesh.add_or_replace_child(skullRot, "skull", CubeListBuilder::create().tex_offs(46, 12).add_box_with([2.0, -2.5, -4.125], [5.0, 9.0, 0.1], CubeDeformation::new(0.0)).tex_offs(46, 12).set_mirror(true).add_box_with([-7.0, -2.5, -4.125], [5.0, 9.0, 0.1], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(0, 38).add_box_with([-5.0, -3.5, -7.125], [10.0, 8.0, 7.0], CubeDeformation::new(0.02)).tex_offs(46, 0).add_box_with([-2.5, 0.5, -9.125], [5.0, 6.0, 6.0], CubeDeformation::new(0.0)).tex_offs(62, 0).add_box_with([-2.5, 0.5, -9.125], [5.0, 4.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let sleep = mesh.add_or_replace_child(skull, "sleep", CubeListBuilder::create().tex_offs(6, 59).add_box_with([0.95, -21.05, -21.525], [2.1, 1.1, 1.0], CubeDeformation::new(0.0)).tex_offs(6, 59).set_mirror(true).add_box_with([-3.05, -21.05, -21.525], [2.1, 1.1, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 20.5, 14.35));
    let right_ears = mesh.add_or_replace_child(skull, "right_ears", CubeListBuilder::create().tex_offs(0, 19).set_mirror(true).add_box_with([-1.5, -2.5, -0.5], [3.0, 3.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-4.5, -3.0, -4.625));
    let left_ears = mesh.add_or_replace_child(skull, "left_ears", CubeListBuilder::create().tex_offs(0, 19).add_box_with([-1.5, -2.5, -0.5], [3.0, 3.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(4.5, -3.0, -4.625));
    let tail1 = mesh.add_or_replace_child(body, "tail1", CubeListBuilder::create().tex_offs(18, 0).add_box_with([-1.0, 0.0, -1.0], [2.0, 8.0, 2.0], CubeDeformation::new(0.02)), PartPose::offset_and_rotation(0.0, -4.4, 11.5, 1.5708, 0.0, 0.0));
    let tail2 = mesh.add_or_replace_child(tail1, "tail2", CubeListBuilder::create().tex_offs(16, 17).add_box_with([-1.0, 0.0, -1.0], [2.0, 5.0, 2.0], CubeDeformation::new(0.0)).tex_offs(18, 14).add_box_with([-1.0, 5.0, -1.0], [2.0, 1.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 8.0, 0.0));
    let tail3 = mesh.add_or_replace_child(tail2, "tail3", CubeListBuilder::create().tex_offs(16, 17).add_box_with([-1.0, 0.0, -1.0], [2.0, 5.0, 2.0], CubeDeformation::new(0.0)).tex_offs(18, 14).add_box_with([-1.0, 5.0, -1.0], [2.0, 1.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 6.0, 0.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(0.0, -22.0, -15.0));
    let front_legs = mesh.add_or_replace_child(legs, "front_legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.1, 1.0));
    let right_arm = mesh.add_or_replace_child(front_legs, "right_arm", CubeListBuilder::create().tex_offs(34, 38).set_mirror(true).add_box_with([-2.0, -1.9, -2.5], [4.0, 16.0, 5.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-3.75, 7.9, 5.5));
    let left_arm = mesh.add_or_replace_child(front_legs, "left_arm", CubeListBuilder::create().tex_offs(34, 38).add_box_with([-2.0, -1.9, -2.5], [4.0, 16.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(3.75, 7.9, 5.5));
    let back_legs = mesh.add_or_replace_child(legs, "back_legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 5.0));
    let right_leg = mesh.add_or_replace_child(back_legs, "right_leg", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-2.0, -1.5, -1.5], [4.0, 14.0, 5.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-3.1, 9.5, 18.5));
    let left_leg = mesh.add_or_replace_child(back_legs, "left_leg", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-2.0, -1.5, -1.5], [4.0, 14.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(3.1, 9.5, 18.5));
    LayerDefinition::create(mesh, 128, 128).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 16.0, 0.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let leftArm = mesh.add_or_replace_child(legs, "leftArm", CubeListBuilder::create().tex_offs(0, 35).add_box_with([-1.0, -1.0, -1.5], [2.0, 10.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(2.5, -1.0, -4.5));
    let rightArm = mesh.add_or_replace_child(legs, "rightArm", CubeListBuilder::create().tex_offs(0, 35).set_mirror(true).add_box_with([-1.0, -1.0, -1.5], [2.0, 10.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-2.5, -1.0, -4.5));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create().tex_offs(10, 35).add_box_with([-1.0, 0.5, -1.5], [2.0, 7.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(2.0, 0.5, 5.5));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create().tex_offs(10, 35).set_mirror(true).add_box_with([-1.0, 0.5, -1.5], [2.0, 7.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-2.0, 0.5, 5.5));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-3.0, -4.0, -8.0], [6.0, 7.0, 15.0], CubeDeformation::new(0.026)), PartPose::offset(0.0, -2.0, 0.0));
    let skull = mesh.add_or_replace_child(body, "skull", CubeListBuilder::create().tex_offs(42, 0).add_box_with([-2.5, -2.3, -6.4], [5.0, 4.0, 2.0], CubeDeformation::new(0.0)).tex_offs(42, 6).add_box_with([-2.5, -1.3, -6.4], [5.0, 3.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.7, -6.6));
    let awake = mesh.add_or_replace_child(skull, "awake", CubeListBuilder::create().tex_offs(0, 22).add_box_with([-4.0, -17.0, -11.0], [8.0, 7.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 11.7, 6.6));
    let leftEar = mesh.add_or_replace_child(skull, "leftEar", CubeListBuilder::create().tex_offs(20, 35).add_box_with([-1.5, -2.5, -0.5], [3.0, 3.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(3.5, -4.8, 0.1));
    let rightEar = mesh.add_or_replace_child(skull, "rightEar", CubeListBuilder::create().tex_offs(20, 35).set_mirror(true).add_box_with([-1.5, -2.5, -0.5], [3.0, 3.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-3.5, -4.8, 0.1));
    let asleep = mesh.add_or_replace_child(skull, "asleep", CubeListBuilder::create().tex_offs(0, 48).add_box_with([-4.0, -17.0, -11.0], [8.0, 7.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 11.7, 6.6));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(28, 22).add_box_with([-0.5, -0.5, 0.0], [1.0, 1.0, 10.0], CubeDeformation::new(0.0)).tex_offs(28, 33).add_box_with([-0.5, -0.5, 0.0], [1.0, 1.0, 10.0], CubeDeformation::new(0.25)), PartPose::offset(0.0, -3.5, 7.0));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
