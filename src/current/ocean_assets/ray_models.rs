// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-5.5, -2.0, -7.0], [11.0, 4.0, 15.0], CubeDeformation::new(0.0)).tex_offs(38, 4).add_box_with([0.0, -3.0, -1.0], [0.0, 1.0, 9.0], CubeDeformation::new(0.0)).tex_offs(23, 43).add_box_with([-2.5, -1.0, -10.0], [5.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -2.0, 0.0));
    let mantaMouth = mesh.add_or_replace_child(body, "mantaMouth", CubeListBuilder::create(), PartPose::offset(4.5, -0.5, -7.0));
    let leftMouth = mesh.add_or_replace_child(mantaMouth, "leftMouth", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.0, -1.5, -4.0], [2.0, 3.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let rightMouth = mesh.add_or_replace_child(mantaMouth, "rightMouth", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-1.0, -1.5, -4.0], [2.0, 3.0, 4.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-9.0, 0.0, 0.0));
    let leftFin = mesh.add_or_replace_child(body, "leftFin", CubeListBuilder::create().tex_offs(0, 29).add_box_with([0.0, 0.0, -5.0], [6.0, 2.0, 10.0], CubeDeformation::new(0.01)).tex_offs(49, 26).add_box_with([0.0, 0.0, -7.0], [6.0, 2.0, 14.0], CubeDeformation::new(0.0)), PartPose::offset(5.5, 0.0, 0.0));
    let leftFin2 = mesh.add_or_replace_child(leftFin, "leftFin2", CubeListBuilder::create().tex_offs(0, 19).add_box_with([0.0, 0.0, -4.0], [13.0, 1.0, 8.0], CubeDeformation::new(0.0)).tex_offs(51, 9).add_box_with([0.0, 0.0, -5.0], [5.0, 1.0, 12.0], CubeDeformation::new(0.0)), PartPose::offset(6.0, 0.0, -1.0));
    let rightFin = mesh.add_or_replace_child(body, "rightFin", CubeListBuilder::create().tex_offs(0, 29).set_mirror(true).add_box_with([-6.0, 0.0, -5.0], [6.0, 2.0, 10.0], CubeDeformation::new(0.01)).set_mirror(false).tex_offs(49, 26).set_mirror(true).add_box_with([-6.0, 0.0, -7.0], [6.0, 2.0, 14.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-5.5, 0.0, 0.0));
    let rightFin2 = mesh.add_or_replace_child(rightFin, "rightFin2", CubeListBuilder::create().tex_offs(0, 19).set_mirror(true).add_box_with([-13.0, 0.0, -4.0], [13.0, 1.0, 8.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(51, 9).set_mirror(true).add_box_with([-5.0, 0.0, -5.0], [5.0, 1.0, 12.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-6.0, 0.0, -1.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(23, 29).add_box_with([-1.5, 0.0, 0.0], [3.0, 2.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -2.0, 8.0));
    let tail2 = mesh.add_or_replace_child(tail, "tail2", CubeListBuilder::create().tex_offs(37, 0).add_box_with([-0.5, -0.5, 0.0], [1.0, 1.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 1.0, 7.0));
    let tail3 = mesh.add_or_replace_child(tail2, "tail3", CubeListBuilder::create().tex_offs(54, 0).add_box_with([-0.5, -0.5, 0.0], [1.0, 1.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 7.0));
    LayerDefinition::create(mesh, 128, 64).bake_root()
}
