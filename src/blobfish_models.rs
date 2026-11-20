// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn pink() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 21.5, -1.25));
    let sf_head = mesh.add_or_replace_child(root, "sf_head", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-5.0, -3.0, -5.0], [10.0, 7.0, 5.0], CubeDeformation::new(0.01)), PartPose::offset(0.0, -1.5, -1.75));
    let nose = mesh.add_or_replace_child(sf_head, "nose", CubeListBuilder::create().tex_offs(31, 5).add_box_with([-2.0, -2.0, -2.0], [4.0, 5.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.0, -5.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 13).add_box_with([-4.0, -3.0, -0.5], [8.0, 6.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -0.5, -2.25));
    let leftFin = mesh.add_or_replace_child(body, "leftFin", CubeListBuilder::create().tex_offs(7, 28).add_box_with([0.0, 0.0, -2.0], [4.0, 0.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(4.0, 3.0, 1.5, -1.5708, 0.0, 0.0));
    let rightFin = mesh.add_or_replace_child(body, "rightFin", CubeListBuilder::create().tex_offs(7, 28).set_mirror(true).add_box_with([-4.0, 0.0, -2.0], [4.0, 0.0, 4.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-4.0, 3.0, 1.5, -1.5708, 0.0, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create(), PartPose::offset(0.0, 1.0, 6.5));
    let tail_r1 = mesh.add_or_replace_child(tail, "tail_r1", CubeListBuilder::create().tex_offs(-4, 28).add_box_with([-3.0, 0.0, 0.0], [6.0, 0.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, 0.0, 1.5708));
    LayerDefinition::create(mesh, 64, 32).bake_root()
}
pub(super) fn gray() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 21.0, -1.0));
    let sf_head = mesh.add_or_replace_child(root, "sf_head", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-4.5, -3.0, -5.0], [9.0, 6.0, 5.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, 0.0, -2.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 12).add_box_with([-3.5, -3.0, 0.0], [7.0, 6.0, 6.0], CubeDeformation::new(0.0)).tex_offs(23, 7).add_box_with([0.0, -5.0, 0.0], [0.0, 2.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, -2.0));
    let leftFin = mesh.add_or_replace_child(body, "leftFin", CubeListBuilder::create().tex_offs(24, 0).add_box_with([0.0, -2.0, 0.0], [5.0, 4.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(3.5, 3.0, 1.0));
    let rightFin = mesh.add_or_replace_child(body, "rightFin", CubeListBuilder::create().tex_offs(24, 0).set_mirror(true).add_box_with([-5.0, -2.0, 0.0], [5.0, 4.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-3.5, 3.0, 1.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(27, 10).add_box_with([0.0, -3.5, 0.0], [0.0, 7.0, 8.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 6.0));
    LayerDefinition::create(mesh, 64, 32).bake_root()
}
