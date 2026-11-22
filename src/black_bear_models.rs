// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, -0.8));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(33, 1).add_box_with([-6.0, -6.2, -9.4], [12.0, 12.0, 19.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, -12.8, 0.2));
    let body_r1 = mesh.add_or_replace_child(body, "body_r1", CubeListBuilder::create().tex_offs(71, 104).add_box_with([-6.5, -13.0, -6.5], [11.0, 1.0, 11.0], CubeDeformation::new(0.3)), PartPose::offset_and_rotation(1.0, -0.7, 3.4, 1.5708, 0.0, 0.0));
    let bell = mesh.add_or_replace_child(body, "bell", CubeListBuilder::create(), PartPose::offset(0.0, 3.8, -9.9));
    let bell_r1 = mesh.add_or_replace_child(bell, "bell_r1", CubeListBuilder::create().tex_offs(116, 118).add_box_with([-1.5, -15.0, -5.5], [3.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -2.5, 12.5, 1.5708, 0.0, 0.0));
    let skullRot = mesh.add_or_replace_child(body, "skullRot", CubeListBuilder::create(), PartPose::offset(0.0, 2.6, -9.4));
    let skull = mesh.add_or_replace_child(skullRot, "skull", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-4.5, -2.0, -6.4], [9.0, 7.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -4.8, 0.4));
    let head_angry = mesh.add_or_replace_child(skull, "head_angry", CubeListBuilder::create().tex_offs(0, 14).add_box_with([-4.5, -17.0, -16.0], [9.0, 7.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 15.0, 9.6));
    let sleep = mesh.add_or_replace_child(skull, "sleep", CubeListBuilder::create().tex_offs(1, 1).add_box_with([-3.5, -16.0, -16.0], [2.0, 1.0, 0.0], CubeDeformation::new(0.0)).tex_offs(1, 1).set_mirror(true).add_box_with([1.5, -16.0, -16.0], [2.0, 1.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 15.0, 9.575));
    let left_ear = mesh.add_or_replace_child(skull, "left_ear", CubeListBuilder::create().tex_offs(25, 2).add_box_with([-0.22, -1.36, 0.12], [2.0, 2.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(3.72, -1.64, -3.52));
    let right_ear = mesh.add_or_replace_child(skull, "right_ear", CubeListBuilder::create().tex_offs(25, 2).set_mirror(true).add_box_with([-1.58, -1.36, 0.12], [2.0, 2.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-3.92, -1.64, -3.52));
    let snout = mesh.add_or_replace_child(skull, "snout", CubeListBuilder::create().tex_offs(0, 28).add_box_with([-2.5, -2.0, -4.0], [5.0, 5.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 2.0, -6.4));
    let rightItem = mesh.add_or_replace_child(snout, "rightItem", CubeListBuilder::create(), PartPose::offset_and_rotation(-0.92, 0.4, 0.52, 0.2618, 0.2618, -0.192));
    let nose = mesh.add_or_replace_child(snout, "nose", CubeListBuilder::create().tex_offs(32, 0).add_box_with([-2.0, -2.2, -1.4], [5.0, 2.0, 2.0], CubeDeformation::new(0.02)), PartPose::offset(-0.5, 0.2, -2.6));
    let snout_berries = mesh.add_or_replace_child(snout, "snout_berries", CubeListBuilder::create().tex_offs(0, 48).add_box_with([-2.5, -2.0, -4.0], [5.0, 5.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let snout_honey = mesh.add_or_replace_child(snout, "snout_honey", CubeListBuilder::create().tex_offs(0, 58).add_box_with([-2.5, -2.0, -4.0], [5.0, 5.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let snout_angry = mesh.add_or_replace_child(snout, "snout_angry", CubeListBuilder::create().tex_offs(0, 38).add_box_with([-2.5, -2.0, -4.0], [5.0, 5.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let front_legs = mesh.add_or_replace_child(root, "front_legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.8));
    let left_arm = mesh.add_or_replace_child(front_legs, "left_arm", CubeListBuilder::create().tex_offs(21, 33).add_box_with([-2.0, 0.0, -2.0], [4.0, 7.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(3.0, -7.0, -6.0));
    let left_arm_berries = mesh.add_or_replace_child(left_arm, "left_arm_berries", CubeListBuilder::create().tex_offs(20, 45).add_box_with([-2.0, 0.0, -2.0], [4.0, 7.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let left_arm_honey = mesh.add_or_replace_child(left_arm, "left_arm_honey", CubeListBuilder::create().tex_offs(20, 57).add_box_with([-2.0, 0.0, -2.0], [4.0, 7.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let right_arm = mesh.add_or_replace_child(front_legs, "right_arm", CubeListBuilder::create().tex_offs(21, 33).set_mirror(true).add_box_with([-2.0, 0.0, -2.0], [4.0, 7.0, 4.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-3.0, -7.0, -6.0));
    let right_arm_berries = mesh.add_or_replace_child(right_arm, "right_arm_berries", CubeListBuilder::create().tex_offs(20, 45).set_mirror(true).add_box_with([-2.0, 0.0, -2.0], [4.0, 7.0, 4.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 0.0, 0.0));
    let right_arm_honey = mesh.add_or_replace_child(right_arm, "right_arm_honey", CubeListBuilder::create().tex_offs(20, 57).set_mirror(true).add_box_with([-2.0, 0.0, -2.0], [4.0, 7.0, 4.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 0.0, 0.0));
    let back_legs = mesh.add_or_replace_child(root, "back_legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.8));
    let left_leg = mesh.add_or_replace_child(back_legs, "left_leg", CubeListBuilder::create().tex_offs(77, 8).add_box_with([-2.0, 0.0, -2.0], [4.0, 7.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(3.0, -7.0, 6.0));
    let right_leg = mesh.add_or_replace_child(back_legs, "right_leg", CubeListBuilder::create().tex_offs(77, 8).set_mirror(true).add_box_with([-2.0, 0.0, -2.0], [4.0, 7.0, 4.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-3.0, -7.0, 6.0));
    LayerDefinition::create(mesh, 128, 128).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(2.75, -4.0, 4.0));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create().tex_offs(20, 18).add_box_with([-1.5, 0.0, -1.5], [3.0, 4.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let leftArm = mesh.add_or_replace_child(legs, "leftArm", CubeListBuilder::create().tex_offs(20, 18).add_box_with([-1.5, 0.0, -1.5], [3.0, 4.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, -6.0));
    let rightArm = mesh.add_or_replace_child(legs, "rightArm", CubeListBuilder::create().tex_offs(20, 18).set_mirror(true).add_box_with([-1.5, 0.0, -1.5], [3.0, 4.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-5.5, 0.0, -6.0));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create().tex_offs(20, 18).set_mirror(true).add_box_with([-1.5, 0.0, -1.5], [3.0, 4.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-5.5, 0.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-4.5, -4.0, -5.0], [9.0, 8.0, 10.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -7.0, 1.0));
    let skull = mesh.add_or_replace_child(body, "skull", CubeListBuilder::create().tex_offs(0, 27).set_mirror(true).add_box_with([-4.0, -3.0, -2.5], [2.0, 2.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(0, 18).add_box_with([-3.0, -2.0, -3.5], [6.0, 5.0, 4.0], CubeDeformation::new(0.0)).tex_offs(0, 27).add_box_with([2.0, -3.0, -2.5], [2.0, 2.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, -4.5));
    let nose = mesh.add_or_replace_child(skull, "nose", CubeListBuilder::create().tex_offs(20, 25).add_box_with([-1.5, -1.5, -3.0], [3.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 1.5, -3.5));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
