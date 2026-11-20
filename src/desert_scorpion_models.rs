// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(22, 0).add_box_with([-1.5, -1.0, 2.0], [3.0, 2.0, 2.0], CubeDeformation::new(0.0)).tex_offs(0, 0).add_box_with([-3.5, -1.0, -6.0], [7.0, 2.0, 8.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.0, 3.0));
    let leftArm = mesh.add_or_replace_child(body, "leftArm", CubeListBuilder::create().tex_offs(12, 10).add_box_with([-1.5, 0.0, -1.0], [7.0, 0.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(3.5, 0.25, -5.0, 0.0, 0.3054, 0.0));
    let leftClaw = mesh.add_or_replace_child(leftArm, "leftClaw", CubeListBuilder::create().tex_offs(0, 10).add_box_with([-3.0, -2.25, -6.0], [5.0, 3.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(3.5, 0.0, -1.0, 0.0, -0.2618, 0.0));
    let rightArm = mesh.add_or_replace_child(body, "rightArm", CubeListBuilder::create().tex_offs(12, 10).set_mirror(true).add_box_with([-5.5, 0.0, -1.0], [7.0, 0.0, 4.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-3.5, 0.25, -5.0, 0.0, -0.3054, 0.0));
    let rightClaw = mesh.add_or_replace_child(rightArm, "rightClaw", CubeListBuilder::create().tex_offs(0, 10).set_mirror(true).add_box_with([-2.0, -2.25, -6.0], [5.0, 3.0, 6.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-3.5, 0.0, -1.0, 0.0, 0.2618, 0.0));
    let tail_1 = mesh.add_or_replace_child(body, "tail_1", CubeListBuilder::create().tex_offs(2, 16).add_box_with([0.0, -9.4554, -0.9718], [0.0, 11.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -0.5446, 2.9718));
    let tail_2 = mesh.add_or_replace_child(tail_1, "tail_2", CubeListBuilder::create(), PartPose::offset(0.0, -7.4554, -0.9718));
    let tail_2_r1 = mesh.add_or_replace_child(tail_2, "tail_2_r1", CubeListBuilder::create().tex_offs(16, 24).add_box_with([-2.0, -1.75, -2.0], [4.0, 4.0, 4.0], CubeDeformation::new(0.0)).tex_offs(23, 0).add_box_with([0.0, 2.25, -3.0], [0.0, 2.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -1.0, -2.0, 0.3927, 0.0, 0.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 3.0));
    let frontLeftLeg = mesh.add_or_replace_child(legs, "frontLeftLeg", CubeListBuilder::create(), PartPose::offset(3.5, -0.85, -2.5));
    let frontLeftLeg_r1 = mesh.add_or_replace_child(frontLeftLeg, "frontLeftLeg_r1", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-1.0, -1.5, -0.5], [2.0, 3.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(1.5, 0.575, 0.0, 0.0, 0.0, -0.829));
    let frontRightLeg = mesh.add_or_replace_child(legs, "frontRightLeg", CubeListBuilder::create(), PartPose::offset(-3.5, -0.85, -2.5));
    let frontRightLeg_r1 = mesh.add_or_replace_child(frontRightLeg, "frontRightLeg_r1", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.0, -1.5, -0.5], [2.0, 3.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-1.5, 0.575, 0.0, 0.0, 0.0, 0.829));
    let middleLeftLeg = mesh.add_or_replace_child(legs, "middleLeftLeg", CubeListBuilder::create(), PartPose::offset(3.5, -0.85, -0.5));
    let middleLeftLeg_r1 = mesh.add_or_replace_child(middleLeftLeg, "middleLeftLeg_r1", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-1.0, -1.5, -0.5], [2.0, 3.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(1.5, 0.575, 0.0, 0.0, 0.0, -0.829));
    let middleRightLeg = mesh.add_or_replace_child(legs, "middleRightLeg", CubeListBuilder::create(), PartPose::offset(-3.5, -0.85, -0.5));
    let middleRightLeg_r1 = mesh.add_or_replace_child(middleRightLeg, "middleRightLeg_r1", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.0, -1.5, -0.5], [2.0, 3.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-1.5, 0.575, 0.0, 0.0, 0.0, 0.829));
    let backLeftLeg = mesh.add_or_replace_child(legs, "backLeftLeg", CubeListBuilder::create(), PartPose::offset(3.5, -0.85, -0.5));
    let backLeftLeg_r1 = mesh.add_or_replace_child(backLeftLeg, "backLeftLeg_r1", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-1.0, -1.5, -0.5], [2.0, 3.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(1.5, 0.575, 2.0, 0.0, 0.0, -0.829));
    let backRightLeg = mesh.add_or_replace_child(legs, "backRightLeg", CubeListBuilder::create(), PartPose::offset(-3.5, -0.85, -0.5));
    let backRightLeg_r1 = mesh.add_or_replace_child(backRightLeg, "backRightLeg_r1", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.0, -1.5, -0.5], [2.0, 3.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-1.5, 0.575, 2.0, 0.0, 0.0, 0.829));
    LayerDefinition::create(mesh, 32, 32).bake_root()
}
