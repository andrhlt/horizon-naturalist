// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 22.0, -1.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-0.5, -1.0, -2.0], [1.0, 3.0, 5.0], CubeDeformation::new(0.0)).tex_offs(6, 8).add_box_with([-0.5, -1.0, -3.0], [1.0, 2.0, 1.0], CubeDeformation::new(0.0)).tex_offs(8, -4).add_box_with([0.0, -2.0, -1.0], [0.0, 1.0, 4.0], CubeDeformation::new(0.0)).tex_offs(8, -3).add_box_with([0.0, 2.0, -1.0], [0.0, 1.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(0, 8).add_box_with([0.0, -1.5, 0.0], [0.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.5, 3.0));
    let leftFin = mesh.add_or_replace_child(body, "leftFin", CubeListBuilder::create().tex_offs(8, 3).add_box_with([0.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.5, 1.0, 0.0, 0.3655, -0.7119, -0.5299));
    let rightFin = mesh.add_or_replace_child(body, "rightFin", CubeListBuilder::create().tex_offs(8, 3).set_mirror(true).add_box_with([-2.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-0.5, 1.0, 0.0, 0.3655, 0.7119, 0.5299));
    LayerDefinition::create(mesh, 16, 16).bake_root()
}
pub(super) fn medium() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 21.0, -1.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.0, -2.0, -2.0], [2.0, 4.0, 6.0], CubeDeformation::new(0.0)).tex_offs(17, 6).add_box_with([-1.0, -2.0, -3.0], [2.0, 3.0, 1.0], CubeDeformation::new(0.0)).tex_offs(0, 7).add_box_with([0.0, -3.0, 0.0], [0.0, 1.0, 4.0], CubeDeformation::new(0.0)).tex_offs(8, 7).add_box_with([0.0, 2.0, 0.0], [0.0, 1.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(11, -1).add_box_with([0.0, -1.5, 0.0], [0.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 4.0));
    let leftFin = mesh.add_or_replace_child(body, "leftFin", CubeListBuilder::create().tex_offs(1, 3).add_box_with([0.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(1.0, 1.0, 0.0, 0.3655, -0.7119, -0.5299));
    let rightFin = mesh.add_or_replace_child(body, "rightFin", CubeListBuilder::create().tex_offs(1, 3).set_mirror(true).add_box_with([-2.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-1.0, 1.0, 0.0, 0.3655, 0.7119, 0.5299));
    LayerDefinition::create(mesh, 32, 32).bake_root()
}
pub(super) fn large() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 20.25, -3.0));
    let head = mesh.add_or_replace_child(root, "head", CubeListBuilder::create().tex_offs(20, 8).add_box_with([-1.0, -2.25, -4.0], [2.0, 4.0, 1.0], CubeDeformation::new(0.0)).tex_offs(20, 0).add_box_with([-1.5, -2.25, -3.0], [3.0, 5.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.5, -3.0, 0.0], [3.0, 5.0, 7.0], CubeDeformation::new(0.0)).tex_offs(14, 12).add_box_with([0.0, -5.0, 0.0], [0.0, 2.0, 7.0], CubeDeformation::new(0.0)).tex_offs(0, 12).add_box_with([0.0, 2.0, 0.0], [0.0, 2.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.75, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(0, 21).add_box_with([0.0, -2.0, 0.0], [0.0, 4.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -0.5, 7.0));
    let rightFin = mesh.add_or_replace_child(body, "rightFin", CubeListBuilder::create().tex_offs(1, 3).set_mirror(true).add_box_with([-2.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-1.5, 1.0, 2.0, 0.3655, 0.7119, 0.5299));
    let leftFin = mesh.add_or_replace_child(body, "leftFin", CubeListBuilder::create().tex_offs(1, 3).add_box_with([0.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(1.5, 1.0, 2.0, 0.3655, -0.7119, -0.5299));
    LayerDefinition::create(mesh, 32, 32).bake_root()
}
