// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let body = mesh.add_or_replace_child(partdefinition, "body", CubeListBuilder::create().tex_offs(21, 14).set_mirror(true).add_box_with([-3.5, -2.0, -4.5], [7.0, 4.0, 9.0], CubeDeformation::new(0.01)).set_mirror(false), PartPose::offset(0.0, 19.0, -3.5));
    let rightFoot = mesh.add_or_replace_child(body, "rightFoot", CubeListBuilder::create().tex_offs(46, 15).add_box_with([-1.5, 0.0, -2.5], [3.0, 2.0, 3.0], CubeDeformation::new(0.01)), PartPose::offset(-2.0, 2.25, 3.0));
    let leftFoot = mesh.add_or_replace_child(body, "leftFoot", CubeListBuilder::create().tex_offs(46, 15).set_mirror(true).add_box_with([-1.5, 0.0, -2.5], [3.0, 2.0, 3.0], CubeDeformation::new(0.01)).set_mirror(false), PartPose::offset(2.0, 2.25, 3.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(0, 23).add_box_with([-3.0, 0.0, 0.0], [5.0, 2.0, 8.0], CubeDeformation::new(0.0)), PartPose::offset(0.5, -2.0, 4.5));
    let neck = mesh.add_or_replace_child(body, "neck", CubeListBuilder::create().tex_offs(0, 35).add_box_with([-2.0, -1.25, -4.0], [3.0, 3.0, 6.0], CubeDeformation::new(0.0)).tex_offs(27, 27).add_box_with([-2.5, -4.25, -8.0], [4.0, 4.0, 5.0], CubeDeformation::new(0.0)).tex_offs(17, 34).add_box_with([-1.0, -3.25, -11.0], [1.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.5, 2.25, -4.5));
    let held_item = mesh.add_or_replace_child(neck, "held_item", CubeListBuilder::create().tex_offs(20, 37).add_box_with([0.5, -3.0, -19.0], [0.0, 0.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(-1.0, 2.75, 6.0));
    let right_wing = mesh.add_or_replace_child(body, "right_wing", CubeListBuilder::create().tex_offs(0, 11).add_box_with([-6.0, 0.0, 0.0], [6.0, 2.0, 9.0], CubeDeformation::new(0.0)), PartPose::offset(-3.5, 0.0, -4.5));
    let right_wing_tip = mesh.add_or_replace_child(right_wing, "right_wing_tip", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-13.0, 0.0, 0.0], [13.0, 1.0, 9.0], CubeDeformation::new(0.0)), PartPose::offset(-6.0, 0.0, 0.05));
    let left_wing = mesh.add_or_replace_child(body, "left_wing", CubeListBuilder::create().tex_offs(0, 11).set_mirror(true).add_box_with([0.0, 0.0, 0.0], [6.0, 2.0, 9.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(3.5, 0.0, -4.5));
    let left_wing_tip = mesh.add_or_replace_child(left_wing, "left_wing_tip", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([0.0, 0.0, 0.0], [13.0, 1.0, 9.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(6.0, 0.0, 0.1));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
