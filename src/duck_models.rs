// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-3.0, -2.0, -3.5], [6.0, 4.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -6.0, -0.5));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(27, 5).add_box_with([-2.0, -1.0, 0.0], [4.0, 2.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 1.0, 3.5));
    let neck = mesh.add_or_replace_child(body, "neck", CubeListBuilder::create().tex_offs(52, 11).add_box_with([-1.5, -7.0, -1.5], [3.0, 8.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.0, -3.0));
    let bill = mesh.add_or_replace_child(neck, "bill", CubeListBuilder::create().tex_offs(26, 0).add_box_with([-1.5, -0.5, -3.0], [3.0, 1.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -4.5, -1.5));
    let bowtie = mesh.add_or_replace_child(neck, "bowtie", CubeListBuilder::create().tex_offs(0, 19).add_box_with([-3.0, -1.5, 0.0], [6.0, 3.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.5, -1.75));
    let leftWing = mesh.add_or_replace_child(body, "leftWing", CubeListBuilder::create().tex_offs(42, 0).add_box_with([0.0, 0.0, -3.0], [1.0, 4.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(3.0, -2.0, 0.5));
    let rightWing = mesh.add_or_replace_child(body, "rightWing", CubeListBuilder::create().tex_offs(42, 0).set_mirror(true).add_box_with([-1.0, 0.0, -3.0], [1.0, 4.0, 6.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-3.0, -2.0, 0.5));
    let leftLeg = mesh.add_or_replace_child(root, "leftLeg", CubeListBuilder::create().tex_offs(19, 0).add_box_with([-1.5, 0.0, 0.0], [3.0, 4.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(1.5, -4.0, 1.0));
    let leftFoot = mesh.add_or_replace_child(leftLeg, "leftFoot", CubeListBuilder::create().tex_offs(29, 12).add_box_with([-1.5, 0.0, -3.0], [3.0, 0.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 4.0, 0.0));
    let rightLeg = mesh.add_or_replace_child(root, "rightLeg", CubeListBuilder::create().tex_offs(38, 0).set_mirror(true).add_box_with([-1.5, 0.0, 0.0], [3.0, 4.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-1.5, -4.0, 1.0));
    let rightFoot = mesh.add_or_replace_child(rightLeg, "rightFoot", CubeListBuilder::create().tex_offs(29, 12).set_mirror(true).add_box_with([-1.5, 0.0, -3.0], [3.0, 0.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 4.0, 0.0));
    LayerDefinition::create(mesh, 64, 32).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 23.75, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 6).add_box_with([-1.5, -2.0, -2.5], [3.0, 2.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.0, 1.5));
    let neck = mesh.add_or_replace_child(body, "neck", CubeListBuilder::create().tex_offs(0, 11).add_box_with([-1.0, -1.0, -4.0], [2.0, 1.0, 2.0], CubeDeformation::new(0.0)).tex_offs(0, 0).add_box_with([-2.0, -3.0, -2.0], [4.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -2.0, -1.5));
    let bowtie = mesh.add_or_replace_child(body, "bowtie", CubeListBuilder::create().tex_offs(0, 19).add_box_with([-3.0, -1.5, -0.1], [6.0, 3.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -0.5, -2.5));
    let leftLeg = mesh.add_or_replace_child(root, "leftLeg", CubeListBuilder::create().tex_offs(8, 11).set_mirror(true).add_box_with([-0.5, 0.5, -2.0], [2.0, 1.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(1.0, -1.5, 1.5));
    let rightLeg = mesh.add_or_replace_child(root, "rightLeg", CubeListBuilder::create().tex_offs(8, 11).add_box_with([-1.5, 0.5, -2.0], [2.0, 1.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(-1.0, -1.5, 1.5));
    LayerDefinition::create(mesh, 64, 32).bake_root()
}
