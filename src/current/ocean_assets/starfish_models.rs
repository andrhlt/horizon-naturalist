// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset_and_rotation(0.0, 24.0, 0.0, 0.0, 3.1416, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-3.0, -1.0, -3.0], [6.0, 3.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -2.0, 0.0));
    let skull = mesh.add_or_replace_child(body, "skull", CubeListBuilder::create(), PartPose::offset(0.0, 1.0, 3.0));
    let skull_r1 = mesh.add_or_replace_child(skull, "skull_r1", CubeListBuilder::create().tex_offs(19, 5).add_box_with([-2.0, -2.0, -11.0], [4.0, 2.0, 5.0], CubeDeformation::new(0.02)), PartPose::offset_and_rotation(0.0, 1.0, -6.0, 0.0, 3.1416, 0.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(-3.0, 0.0, 1.25));
    let leftArm = mesh.add_or_replace_child(legs, "leftArm", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let leftArm_r1 = mesh.add_or_replace_child(leftArm, "leftArm_r1", CubeListBuilder::create().tex_offs(16, 13).set_mirror(true).add_box_with([-2.0, -2.0, -4.5], [4.0, 2.0, 6.0], CubeDeformation::new(0.01)).set_mirror(false), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, 1.9635, 0.0));
    let rightArm = mesh.add_or_replace_child(legs, "rightArm", CubeListBuilder::create(), PartPose::offset(6.0, 0.0, 0.0));
    let rightArm_r1 = mesh.add_or_replace_child(rightArm, "rightArm_r1", CubeListBuilder::create().tex_offs(16, 13).add_box_with([-2.0, -2.0, -4.5], [4.0, 2.0, 6.0], CubeDeformation::new(0.01)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, -1.9635, 0.0));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create(), PartPose::offset(1.0, 0.0, -3.5));
    let leftLeg_r1 = mesh.add_or_replace_child(leftLeg, "leftLeg_r1", CubeListBuilder::create().tex_offs(1, 11).set_mirror(true).add_box_with([-2.0, -2.0, -0.75], [4.0, 2.0, 6.0], CubeDeformation::new(0.02)).set_mirror(false), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, -2.3562, 0.0));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create(), PartPose::offset(5.0, 0.0, -3.5));
    let rightLeg_r1 = mesh.add_or_replace_child(rightLeg, "rightLeg_r1", CubeListBuilder::create().tex_offs(1, 11).add_box_with([-2.0, -2.0, -0.75], [4.0, 2.0, 6.0], CubeDeformation::new(0.02)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, 2.3562, 0.0));
    LayerDefinition::create(mesh, 48, 32).bake_root()
}
