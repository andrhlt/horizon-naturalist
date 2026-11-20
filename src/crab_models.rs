// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let head = mesh.add_or_replace_child(root, "head", CubeListBuilder::create(), PartPose::offset(0.0, 6.5, 0.0));
    let crabBody = mesh.add_or_replace_child(head, "crabBody", CubeListBuilder::create(), PartPose::offset(0.5, -8.75, -1.0));
    let shell = mesh.add_or_replace_child(crabBody, "shell", CubeListBuilder::create().tex_offs(0, 18).add_box_with([-3.0, -5.0, -1.0], [5.0, 7.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.0, 0.0));
    let crabHead = mesh.add_or_replace_child(crabBody, "crabHead", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.5, -1.0, -2.0], [3.0, 2.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(-0.5, -0.25, -1.0));
    let leftEye = mesh.add_or_replace_child(crabHead, "leftEye", CubeListBuilder::create().tex_offs(0, 6).set_mirror(true).add_box_with([-0.5, -3.0, 0.0], [2.0, 3.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(1.0, -1.0, -2.0));
    let rightEye = mesh.add_or_replace_child(crabHead, "rightEye", CubeListBuilder::create().tex_offs(0, 6).add_box_with([-1.5, -3.0, 0.0], [2.0, 3.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(-1.0, -1.0, -2.0));
    let leftClaw = mesh.add_or_replace_child(crabBody, "leftClaw", CubeListBuilder::create().tex_offs(0, 10).set_mirror(true).add_box_with([-0.5, -1.0, -3.0], [3.0, 2.0, 4.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(1.5, 0.75, -2.0));
    let rightClaw = mesh.add_or_replace_child(crabBody, "rightClaw", CubeListBuilder::create().tex_offs(0, 10).add_box_with([-2.5, -1.0, -3.0], [3.0, 2.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(-2.5, 0.75, -2.0));
    let rightArm = mesh.add_or_replace_child(rightClaw, "rightArm", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let rightItem = mesh.add_or_replace_child(rightArm, "rightItem", CubeListBuilder::create(), PartPose::offset_and_rotation(-1.0, -1.0, -2.0, 0.0, 1.5708, 0.0));
    let legs = mesh.add_or_replace_child(head, "legs", CubeListBuilder::create(), PartPose::offset(-1.5, -8.0, -1.5));
    let rightLegs = mesh.add_or_replace_child(legs, "rightLegs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.5));
    let crabRightArm = mesh.add_or_replace_child(rightLegs, "crabRightArm", CubeListBuilder::create().tex_offs(9, 2).set_mirror(true).add_box_with([-4.0, 0.0, -0.5], [4.0, 0.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(0.0, 0.0, -0.5, 0.0, 0.0, -0.3927));
    let crabRightLeg = mesh.add_or_replace_child(rightLegs, "crabRightLeg", CubeListBuilder::create().tex_offs(9, 2).set_mirror(true).add_box_with([-4.0, 0.0, -0.5], [4.0, 0.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(0.0, 0.0, 0.5, 0.0, 0.3927, -0.3927));
    let leftLegs = mesh.add_or_replace_child(legs, "leftLegs", CubeListBuilder::create(), PartPose::offset(3.0, 0.0, 0.5));
    let crabLeftLeg = mesh.add_or_replace_child(leftLegs, "crabLeftLeg", CubeListBuilder::create().tex_offs(9, 2).add_box_with([0.0, 0.0, -0.5], [4.0, 0.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.5, 0.0, -0.3927, 0.3927));
    let crabLeftArm = mesh.add_or_replace_child(leftLegs, "crabLeftArm", CubeListBuilder::create().tex_offs(9, 2).add_box_with([0.0, 0.0, -0.5], [4.0, 0.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, -0.5, 0.0, 0.0, 0.3927));
    LayerDefinition::create(mesh, 32, 32).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let crabBody = mesh.add_or_replace_child(root, "crabBody", CubeListBuilder::create(), PartPose::offset(0.0, -1.5, -0.5));
    let crabTorso = mesh.add_or_replace_child(crabBody, "crabTorso", CubeListBuilder::create().tex_offs(0, 3).add_box_with([-1.0, 0.0, -2.0], [2.0, 1.0, 3.0], CubeDeformation::new(0.026)), PartPose::offset(0.0, -0.5, 0.5));
    let shell = mesh.add_or_replace_child(crabBody, "shell", CubeListBuilder::create().tex_offs(0, 12).add_box_with([-1.5, -4.0, -0.5], [3.0, 4.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, -0.25));
    let leftClaw = mesh.add_or_replace_child(crabBody, "leftClaw", CubeListBuilder::create().tex_offs(0, 12).add_box_with([0.0, -0.5, -0.5], [1.0, 1.0, 0.0], CubeDeformation::new(0.0)).tex_offs(6, 7).add_box_with([0.0, -0.5, -1.5], [1.0, 1.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(1.0, 0.5, -1.5));
    let rightClaw = mesh.add_or_replace_child(crabBody, "rightClaw", CubeListBuilder::create().tex_offs(0, 12).set_mirror(true).add_box_with([-1.0, -0.5, -0.5], [1.0, 1.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(6, 7).set_mirror(true).add_box_with([-1.0, -0.5, -1.5], [1.0, 1.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-1.0, 0.5, -1.5));
    let leftEye = mesh.add_or_replace_child(crabBody, "leftEye", CubeListBuilder::create().tex_offs(0, 7).add_box_with([-1.5, -3.0, 0.0], [3.0, 3.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -0.5, -1.5));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(0.0, -1.5, -0.5));
    let crabLeftArm = mesh.add_or_replace_child(legs, "crabLeftArm", CubeListBuilder::create(), PartPose::offset(1.0, 0.5, 0.0));
    let crabLeftArm_r1 = mesh.add_or_replace_child(crabLeftArm, "crabLeftArm_r1", CubeListBuilder::create().tex_offs(0, 10).add_box_with([0.0, 0.0, -0.5], [2.0, 0.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, 0.0, 0.5236));
    let crabRightArm = mesh.add_or_replace_child(legs, "crabRightArm", CubeListBuilder::create(), PartPose::offset(-1.0, 0.5, 0.0));
    let crabRightArm_r1 = mesh.add_or_replace_child(crabRightArm, "crabRightArm_r1", CubeListBuilder::create().tex_offs(0, 10).set_mirror(true).add_box_with([-2.0, 0.0, -0.5], [2.0, 0.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, 0.0, -0.5236));
    let crabLeftLeg = mesh.add_or_replace_child(legs, "crabLeftLeg", CubeListBuilder::create(), PartPose::offset_and_rotation(1.0, 0.5, 1.0, 0.0, -0.7854, 0.0));
    let crabLeftLeg_r1 = mesh.add_or_replace_child(crabLeftLeg, "crabLeftLeg_r1", CubeListBuilder::create().tex_offs(0, 10).add_box_with([0.0, 0.0, -0.5], [2.0, 0.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, 0.0, 0.5236));
    let crabRightLeg = mesh.add_or_replace_child(legs, "crabRightLeg", CubeListBuilder::create(), PartPose::offset_and_rotation(-1.0, 0.5, 1.0, 0.0, 0.7854, 0.0));
    let crabRightLeg_r1 = mesh.add_or_replace_child(crabRightLeg, "crabRightLeg_r1", CubeListBuilder::create().tex_offs(0, 10).set_mirror(true).add_box_with([-2.0, 0.0, -0.5], [2.0, 0.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, 0.0, -0.5236));
    LayerDefinition::create(mesh, 32, 32).bake_root()
}
