// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 13.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let body_r1 = mesh.add_or_replace_child(body, "body_r1", CubeListBuilder::create().tex_offs(36, 25).add_box_with([0.0, -23.0, 4.0], [0.0, 16.0, 4.0], CubeDeformation::new(0.0)).tex_offs(0, 0).add_box_with([-5.0, -23.0, -5.0], [10.0, 16.0, 9.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -1.0, 15.0, 1.5708, 0.0, 0.0));
    let neck = mesh.add_or_replace_child(body, "neck", CubeListBuilder::create(), PartPose::offset(0.0, -2.0, -9.0));
    let neck_r1 = mesh.add_or_replace_child(neck, "neck_r1", CubeListBuilder::create().tex_offs(0, 25).add_box_with([-4.0, -14.7071, -18.2929], [8.0, 5.0, 10.0], CubeDeformation::new(0.0)).tex_offs(0, 40).add_box_with([-6.0, -14.7071, -14.2929], [2.0, 5.0, 4.0], CubeDeformation::new(0.0)).tex_offs(0, 40).set_mirror(true).add_box_with([4.0, -14.7071, -14.2929], [2.0, 5.0, 4.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(38, 20).set_mirror(true).add_box_with([-6.5, -18.705, -8.8772], [3.0, 4.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(38, 20).add_box_with([3.5, -18.705, -8.8772], [3.0, 4.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 2.8284, 15.5564, 0.7854, 0.0, 0.0));
    let tusks = mesh.add_or_replace_child(neck, "tusks", CubeListBuilder::create(), PartPose::offset_and_rotation(0.0, 3.2426, -5.6569, 1.5708, 0.0, 0.0));
    let tusks_r1 = mesh.add_or_replace_child(tusks, "tusks_r1", CubeListBuilder::create().tex_offs(38, 11).add_box_with([4.0, 2.9076, -5.1931], [2.0, 2.0, 7.0], CubeDeformation::new(0.0)).tex_offs(38, 11).set_mirror(true).add_box_with([-4.0, 2.9076, -5.1931], [2.0, 2.0, 7.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-1.0, 4.3424, 1.1931, -2.3562, 0.0, 0.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(3.0, 5.0, -5.0));
    let leftArm = mesh.add_or_replace_child(legs, "leftArm", CubeListBuilder::create().tex_offs(38, 0).add_box_with([-2.0, -1.0, -2.0], [4.0, 7.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create().tex_offs(38, 0).add_box_with([-2.0, -1.0, -2.0], [4.0, 7.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(-6.0, 0.0, 12.0));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create().tex_offs(38, 0).add_box_with([-2.0, -1.0, -2.0], [4.0, 7.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 12.0));
    let rightArm = mesh.add_or_replace_child(legs, "rightArm", CubeListBuilder::create().tex_offs(38, 0).add_box_with([-2.0, -1.0, -2.0], [4.0, 7.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(-6.0, 0.0, 0.0));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 19.0, 1.5));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-3.5, -3.0, -4.5], [7.0, 6.0, 9.0], CubeDeformation::new(0.0)).tex_offs(15, 22).add_box_with([0.0, -9.0, -1.5], [0.0, 6.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let neck = mesh.add_or_replace_child(body, "neck", CubeListBuilder::create().tex_offs(0, 28).add_box_with([0.0, -7.2008, -1.8663], [0.0, 6.0, 4.0], CubeDeformation::new(0.0)).tex_offs(0, 15).add_box_with([-3.5, -3.2008, -3.8663], [7.0, 6.0, 6.0], CubeDeformation::new(0.02)).tex_offs(3, 29).add_box_with([-2.5, -0.2008, -4.8663], [5.0, 2.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.7992, -3.6337));
    let rightEar = mesh.add_or_replace_child(neck, "rightEar", CubeListBuilder::create().tex_offs(0, 3).set_mirror(true).add_box_with([-3.5, -3.9979, -0.5843], [3.0, 4.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-2.0, -1.2008, 0.1337));
    let leftEar = mesh.add_or_replace_child(neck, "leftEar", CubeListBuilder::create().tex_offs(0, 3).add_box_with([0.5, -3.9979, -0.5843], [3.0, 4.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(2.0, -1.2008, 0.1337));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(0.0, 3.0, 0.0));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create().tex_offs(20, 15).set_mirror(true).add_box_with([-1.0, 0.0, -1.0], [2.0, 2.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-2.5, 0.0, 3.5));
    let leftArm = mesh.add_or_replace_child(legs, "leftArm", CubeListBuilder::create().tex_offs(20, 15).add_box_with([-1.0, 0.0, -1.0], [2.0, 2.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(2.5, 0.0, -3.5));
    let rightArm = mesh.add_or_replace_child(legs, "rightArm", CubeListBuilder::create().tex_offs(20, 15).set_mirror(true).add_box_with([-1.0, 0.0, -1.0], [2.0, 2.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-2.5, 0.0, -3.5));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create().tex_offs(20, 15).add_box_with([-1.0, 0.0, -1.0], [2.0, 2.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(2.5, 0.0, 3.5));
    LayerDefinition::create(mesh, 32, 64).bake_root()
}
