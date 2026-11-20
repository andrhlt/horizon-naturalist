// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(48, 3).add_box_with([-3.0, -8.0, 1.0], [6.0, 5.0, 9.0], CubeDeformation::new(0.0)).tex_offs(1, 1).add_box_with([-4.5, -8.0, -10.0], [9.0, 5.0, 11.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -2.0, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(27, 36).add_box_with([-1.5, -1.0, -1.0], [3.0, 3.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -8.0, 10.0));
    let tail2 = mesh.add_or_replace_child(tail, "tail2", CubeListBuilder::create().tex_offs(48, 28).add_box_with([-1.5, -10.0, -1.0], [3.0, 10.0, 3.0], CubeDeformation::new(0.02)), PartPose::offset_and_rotation(0.0, -1.0, 5.75, 0.3927, 0.0, 0.0));
    let stinger = mesh.add_or_replace_child(tail2, "stinger", CubeListBuilder::create().tex_offs(65, 30).add_box_with([-3.0, -4.9002, -5.7549], [5.0, 6.0, 6.0], CubeDeformation::new(0.0)).tex_offs(39, 13).add_box_with([-0.5, -4.1502, -10.7549], [0.0, 6.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.5, -10.0, 0.0, 0.3927, 0.0, 0.0));
    let arms = mesh.add_or_replace_child(body, "arms", CubeListBuilder::create(), PartPose::offset(-0.5, -5.0, -6.0));
    let leftArm = mesh.add_or_replace_child(arms, "leftArm", CubeListBuilder::create().tex_offs(4, 43).add_box_with([-1.5, -1.5, -6.0], [2.0, 2.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(5.5, 0.5, -1.0, 0.0, -0.7854, 0.0));
    let leftClaw = mesh.add_or_replace_child(leftArm, "leftClaw", CubeListBuilder::create().tex_offs(2, 22).add_box_with([-2.0, -4.5, -10.0], [4.0, 8.0, 10.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-0.5, -1.0, -6.0, -0.3927, 0.7854, -0.7854));
    let rightArm = mesh.add_or_replace_child(arms, "rightArm", CubeListBuilder::create().tex_offs(4, 43).set_mirror(true).add_box_with([-0.5, -1.5, -6.0], [2.0, 2.0, 7.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-4.5, 0.5, -1.0, 0.0, 0.7854, 0.0));
    let rightClaw = mesh.add_or_replace_child(rightArm, "rightClaw", CubeListBuilder::create().tex_offs(2, 22).set_mirror(true).add_box_with([-2.0, -4.5, -10.0], [4.0, 8.0, 10.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(0.5, -1.0, -6.0, -0.3927, -0.7854, 0.7854));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(0.0, -5.0, -1.0));
    let leftLegs = mesh.add_or_replace_child(legs, "leftLegs", CubeListBuilder::create(), PartPose::offset_and_rotation(3.0, -0.5, 0.5, 0.0, 0.0, 0.2182));
    let frontLeftLeg = mesh.add_or_replace_child(leftLegs, "frontLeftLeg", CubeListBuilder::create().tex_offs(54, 21).add_box_with([0.0, -1.5, -1.0], [11.0, 2.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, -3.0, 0.0, 0.3927, 0.3054));
    let middleLeftLeg = mesh.add_or_replace_child(leftLegs, "middleLeftLeg", CubeListBuilder::create().tex_offs(54, 21).add_box_with([0.0, -1.5, -1.0], [11.0, 2.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-1.0, 0.0, 0.0, 0.0, 0.0, 0.3054));
    let backLeftLeg = mesh.add_or_replace_child(leftLegs, "backLeftLeg", CubeListBuilder::create().tex_offs(54, 21).add_box_with([0.0, -1.5, -1.0], [11.0, 2.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-1.0, 0.0, 3.0, 0.0, -0.3927, 0.3054));
    let rightLegs = mesh.add_or_replace_child(legs, "rightLegs", CubeListBuilder::create(), PartPose::offset_and_rotation(-3.0, -0.5, 0.5, 0.0, 0.0, -0.2182));
    let frontRightLeg = mesh.add_or_replace_child(rightLegs, "frontRightLeg", CubeListBuilder::create().tex_offs(54, 21).set_mirror(true).add_box_with([-11.0, -1.5, -1.0], [11.0, 2.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(0.0, 0.0, -3.0, 0.0, -0.3927, -0.3054));
    let middleRightLeg = mesh.add_or_replace_child(rightLegs, "middleRightLeg", CubeListBuilder::create().tex_offs(54, 21).set_mirror(true).add_box_with([-11.0, -1.5, -1.0], [11.0, 2.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(1.0, 0.0, 0.0, 0.0, 0.0, -0.3054));
    let backRightLeg = mesh.add_or_replace_child(rightLegs, "backRightLeg", CubeListBuilder::create().tex_offs(54, 21).set_mirror(true).add_box_with([-11.0, -1.5, -1.0], [11.0, 2.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(1.0, 0.0, 3.0, 0.0, 0.3927, -0.3054));
    LayerDefinition::create(mesh, 128, 64).bake_root()
}
