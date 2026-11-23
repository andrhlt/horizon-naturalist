// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 20.0, 1.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 18).add_box_with([-1.0, -3.0, -5.0], [2.0, 6.0, 8.0], CubeDeformation::new(0.0)).tex_offs(12, 12).add_box_with([-1.0, -3.0, -5.0], [2.0, 6.0, 8.0], CubeDeformation::new(0.2)), PartPose::offset(0.0, 0.0, 0.0));
    let jaw = mesh.add_or_replace_child(body, "jaw", CubeListBuilder::create().tex_offs(0, 2).add_box_with([-1.5, -1.0, -4.0], [3.0, 1.0, 5.0], CubeDeformation::new(0.0)).tex_offs(16, 0).add_box_with([-1.5, 0.0, -4.0], [3.0, 2.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 3.0, -2.0));
    let right_fin = mesh.add_or_replace_child(body, "right_fin", CubeListBuilder::create().tex_offs(2, 16).add_box_with([-2.0, -2.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-1.0, 3.0, 0.0, 0.0, 0.7854, 0.0));
    let left_fin = mesh.add_or_replace_child(body, "left_fin", CubeListBuilder::create().tex_offs(2, 12).add_box_with([0.0, -2.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(1.0, 3.0, 0.0, 0.0, -0.7854, 0.0));
    let fin_top = mesh.add_or_replace_child(body, "fin_top", CubeListBuilder::create().tex_offs(20, 2).add_box_with([0.0, -4.0, 0.0], [0.0, 4.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -3.0, -3.0));
    let fin_bottom = mesh.add_or_replace_child(body, "fin_bottom", CubeListBuilder::create().tex_offs(20, 12).add_box_with([0.0, 0.0, 1.0], [0.0, 4.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 3.0, -3.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(6, 3).add_box_with([0.0, -3.0, 0.0], [0.0, 6.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 3.0));
    LayerDefinition::create(mesh, 32, 32).bake_root()
}
