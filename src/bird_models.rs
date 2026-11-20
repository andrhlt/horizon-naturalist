// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let bodyRot = mesh.add_or_replace_child(root, "bodyRot", CubeListBuilder::create(), PartPose::offset(0.0, -6.0, 0.0));
    let body = mesh.add_or_replace_child(bodyRot, "body", CubeListBuilder::create().tex_offs(0, 7).add_box_with([-1.5, -4.0, -2.0], [3.0, 4.0, 4.0], CubeDeformation::new(0.05)).tex_offs(16, 15).add_box_with([-2.0, -4.0, -2.0], [4.0, 4.0, 4.0], CubeDeformation::new(0.05)), PartPose::offset(0.0, 4.0, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(8, 16).add_box_with([-1.5, 0.0, 0.0], [3.0, 4.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 1.0));
    let wings = mesh.add_or_replace_child(body, "wings", CubeListBuilder::create(), PartPose::offset(0.0, -6.0, 0.0));
    let leftWingShoulderRot = mesh.add_or_replace_child(wings, "leftWingShoulderRot", CubeListBuilder::create(), PartPose::offset(1.5, 3.0, 1.5));
    let leftWingRot = mesh.add_or_replace_child(leftWingShoulderRot, "leftWingRot", CubeListBuilder::create(), PartPose::offset(0.5, -1.0, -1.5));
    let leftWing = mesh.add_or_replace_child(leftWingRot, "leftWing", CubeListBuilder::create().tex_offs(14, 6).add_box_with([-1.0, 0.0, -1.5], [1.0, 5.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.5, 0.0, 0.0));
    let rightWingShoulderRot = mesh.add_or_replace_child(wings, "rightWingShoulderRot", CubeListBuilder::create(), PartPose::offset(-1.5, 3.0, 1.5));
    let rightWingRot = mesh.add_or_replace_child(rightWingShoulderRot, "rightWingRot", CubeListBuilder::create(), PartPose::offset(-0.5, -1.0, -1.5));
    let rightWing = mesh.add_or_replace_child(rightWingRot, "rightWing", CubeListBuilder::create().tex_offs(14, 6).set_mirror(true).add_box_with([0.0, 0.0, -1.5], [1.0, 5.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-0.5, 0.0, 0.0));
    let headRot = mesh.add_or_replace_child(body, "headRot", CubeListBuilder::create(), PartPose::offset(0.0, -4.0, 0.0));
    let head = mesh.add_or_replace_child(headRot, "head", CubeListBuilder::create().tex_offs(0, 21).add_box_with([-1.5, -3.0, -1.5], [3.0, 3.0, 3.0], CubeDeformation::new(0.0)).tex_offs(0, 0).add_box_with([-1.0, -3.0, -1.5], [2.0, 3.0, 3.0], CubeDeformation::new(0.0)).tex_offs(12, 1).add_box_with([-0.5, -2.0, -2.5], [1.0, 1.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let comb = mesh.add_or_replace_child(head, "comb", CubeListBuilder::create(), PartPose::offset(0.0, -2.6, 0.0));
    let comb_r1 = mesh.add_or_replace_child(comb, "comb_r1", CubeListBuilder::create().tex_offs(19, 0).add_box_with([0.0, -3.0, 0.0], [0.0, 3.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, -0.3927, 0.0, 0.0));
    let legs = mesh.add_or_replace_child(bodyRot, "legs", CubeListBuilder::create(), PartPose::offset(0.0, 4.0, 0.5));
    let right_leg = mesh.add_or_replace_child(legs, "right_leg", CubeListBuilder::create().tex_offs(16, 0).add_box_with([-0.5, 0.0, -1.0], [1.0, 2.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(-1.0, 0.0, 0.0));
    let left_leg = mesh.add_or_replace_child(legs, "left_leg", CubeListBuilder::create().tex_offs(20, 0).add_box_with([-0.5, 0.0, -1.0], [1.0, 2.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(1.0, 0.0, 0.0));
    LayerDefinition::create(mesh, 32, 32).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let neck = mesh.add_or_replace_child(root, "neck", CubeListBuilder::create().tex_offs(4, 10).add_box_with([-1.5, -2.25, -1.25], [3.0, 3.0, 3.0], CubeDeformation::new(0.3)).tex_offs(0, 2).add_box_with([0.0, -5.25, -1.25], [0.0, 4.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -0.75, 0.0));
    let mouthOpen = mesh.add_or_replace_child(neck, "mouthOpen", CubeListBuilder::create().tex_offs(13, 0).add_box_with([-2.0, -1.0, -2.0], [3.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.5, -1.25, 0.75));
    let mouthClosed = mesh.add_or_replace_child(neck, "mouthClosed", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-2.0, -1.0, -2.0], [3.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.5, -1.25, 0.75));
    let beak = mesh.add_or_replace_child(neck, "beak", CubeListBuilder::create().tex_offs(10, 0).add_box_with([-1.0, 0.0, 0.0], [1.0, 1.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(0.5, -1.25, -2.25));
    let leftWing = mesh.add_or_replace_child(neck, "leftWing", CubeListBuilder::create().tex_offs(-1, 0).add_box_with([0.0, 0.0, 0.0], [1.0, 0.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(1.5, -0.25, -0.25));
    let rightWing = mesh.add_or_replace_child(neck, "rightWing", CubeListBuilder::create().tex_offs(-1, 0).set_mirror(true).add_box_with([-1.0, 0.0, 0.0], [1.0, 0.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-1.5, -0.25, -0.25));
    LayerDefinition::create(mesh, 32, 16).bake_root()
}
