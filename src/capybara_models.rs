// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-4.5, -5.0, -9.0], [9.0, 10.0, 18.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -11.0, 0.0));
    let skull = mesh.add_or_replace_child(body, "skull", CubeListBuilder::create().tex_offs(0, 28).add_box_with([-2.5, -6.0, -9.5], [5.0, 8.0, 11.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -3.0, -7.5));
    let leftEar = mesh.add_or_replace_child(skull, "leftEar", CubeListBuilder::create().tex_offs(11, 0).add_box_with([-0.45, -1.5, 0.0], [1.0, 3.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(2.15, -4.5, 0.9, 0.0, 0.7854, 0.0));
    let rightEar = mesh.add_or_replace_child(skull, "rightEar", CubeListBuilder::create().tex_offs(11, 0).set_mirror(true).add_box_with([-0.55, -1.5, 0.0], [1.0, 3.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-2.15, -4.5, 0.9, 0.0, -0.7854, 0.0));
    let leftArm = mesh.add_or_replace_child(root, "leftArm", CubeListBuilder::create().tex_offs(32, 28).add_box_with([-1.5, -2.5, -2.0], [3.0, 9.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(2.0, -6.5, -5.0));
    let rightArm = mesh.add_or_replace_child(root, "rightArm", CubeListBuilder::create().tex_offs(32, 28).set_mirror(true).add_box_with([-1.5, -2.5, -2.0], [3.0, 9.0, 4.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-2.0, -6.5, -5.0));
    let leftLeg = mesh.add_or_replace_child(root, "leftLeg", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.5, -2.0, -2.5], [3.0, 10.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(2.0, -8.0, 8.5));
    let rightLeg = mesh.add_or_replace_child(root, "rightLeg", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-1.5, -2.0, -2.5], [3.0, 10.0, 5.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-2.0, -8.0, 8.5));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(-0.75, 19.25, 1.75));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(-0.75, 1.25, 2.25));
    let backLegs = mesh.add_or_replace_child(legs, "backLegs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let rightLeg = mesh.add_or_replace_child(backLegs, "rightLeg", CubeListBuilder::create().tex_offs(18, 15).set_mirror(true).add_box_with([-1.0, -0.5, -1.0], [2.0, 4.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 0.0, 0.0));
    let leftLeg = mesh.add_or_replace_child(backLegs, "leftLeg", CubeListBuilder::create().tex_offs(18, 15).add_box_with([-1.0, -0.5, -1.0], [2.0, 4.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(3.0, 0.0, 0.0));
    let frontLegs = mesh.add_or_replace_child(legs, "frontLegs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, -7.0));
    let rightArm = mesh.add_or_replace_child(frontLegs, "rightArm", CubeListBuilder::create().tex_offs(18, 15).set_mirror(true).add_box_with([-1.0, -0.5, -1.0], [2.0, 4.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 0.0, 0.0));
    let leftArm = mesh.add_or_replace_child(frontLegs, "leftArm", CubeListBuilder::create().tex_offs(18, 15).add_box_with([-1.0, -0.5, -1.0], [2.0, 4.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(3.0, 0.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-3.0, -3.0, -4.5], [6.0, 6.0, 9.0], CubeDeformation::new(0.0)), PartPose::offset(0.75, -1.25, -2.25));
    let skull = mesh.add_or_replace_child(body, "skull", CubeListBuilder::create().tex_offs(0, 15).add_box_with([-1.5, -3.5, -5.5], [3.0, 5.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.5, -4.0));
    let asleep = mesh.add_or_replace_child(skull, "asleep", CubeListBuilder::create().tex_offs(12, 21).add_box_with([-1.5, -11.0, -10.0], [3.0, 5.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 7.5, 4.5));
    let leftEar = mesh.add_or_replace_child(skull, "leftEar", CubeListBuilder::create().tex_offs(16, 20).add_box_with([0.0, -0.5, 0.0], [1.0, 1.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(1.5, -3.0, 0.5));
    let rightEar = mesh.add_or_replace_child(skull, "rightEar", CubeListBuilder::create().tex_offs(16, 20).set_mirror(true).add_box_with([-1.0, -0.5, 0.0], [1.0, 1.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-1.5, -3.0, 0.5));
    LayerDefinition::create(mesh, 48, 48).bake_root()
}
