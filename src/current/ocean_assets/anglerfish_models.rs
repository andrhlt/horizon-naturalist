// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-4.0, -6.0, -5.0], [8.0, 12.0, 10.0], CubeDeformation::new(0.0)).tex_offs(44, 48).add_box_with([-3.5, -6.0, -7.0], [7.0, 12.0, 2.0], CubeDeformation::new(-0.1)), PartPose::offset(0.0, -6.0, -1.0));
    let dangly = mesh.add_or_replace_child(body, "dangly", CubeListBuilder::create().tex_offs(0, 30).add_box_with([0.0, -9.0, -8.5], [0.0, 9.0, 10.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -6.0, -3.5, 0.3927, 0.0, 0.0));
    let bulb = mesh.add_or_replace_child(dangly, "bulb", CubeListBuilder::create().tex_offs(30, 49).add_box_with([-1.5, -1.0, -2.0], [3.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -5.231, -7.4043, -0.3927, 0.0, 0.0));
    let jaw = mesh.add_or_replace_child(body, "jaw", CubeListBuilder::create().tex_offs(0, 23).add_box_with([-4.0, -12.0, -2.0], [8.0, 12.0, 2.0], CubeDeformation::new(0.0)).tex_offs(45, 24).add_box_with([-3.5, -12.0, 0.0], [7.0, 12.0, 2.0], CubeDeformation::new(0.02)), PartPose::offset_and_rotation(0.0, 6.0, -5.0, 0.3927, 0.0, 0.0));
    let leftFin = mesh.add_or_replace_child(body, "leftFin", CubeListBuilder::create().tex_offs(27, 0).add_box_with([0.0, -3.5, 0.0], [7.0, 7.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(4.0, 0.5, 1.0));
    let rightFin = mesh.add_or_replace_child(body, "rightFin", CubeListBuilder::create().tex_offs(27, 0).set_mirror(true).add_box_with([-7.0, -3.5, 0.0], [7.0, 7.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-4.0, 0.5, 1.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(32, 35).add_box_with([-2.0, -4.0, 0.125], [4.0, 8.0, 4.0], CubeDeformation::new(0.0)).tex_offs(37, 2).add_box_with([0.0, -7.0, 0.125], [0.0, 3.0, 6.0], CubeDeformation::new(0.0)).tex_offs(31, 17).add_box_with([0.0, 4.0, 0.125], [0.0, 3.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 4.875));
    let tailFin = mesh.add_or_replace_child(tail, "tailFin", CubeListBuilder::create().tex_offs(21, 23).add_box_with([0.0, -4.0, 0.0], [0.0, 8.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 4.125));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
