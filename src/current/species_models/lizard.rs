// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 20.25, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-3.0, -2.0, -5.0], [6.0, 4.0, 10.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.75, 0.0));
    let beardie_body = mesh.add_or_replace_child(body, "beardie_body", CubeListBuilder::create().tex_offs(28, 20).add_box_with([-5.0, -6.0, -5.0], [10.0, 0.0, 10.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 6.0, 0.0));
    let basilisk_body = mesh.add_or_replace_child(body, "basilisk_body", CubeListBuilder::create().tex_offs(33, 0).add_box_with([0.0, -12.0, -5.0], [0.0, 4.0, 10.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 6.0, 0.0));
    let skullRot = mesh.add_or_replace_child(body, "skullRot", CubeListBuilder::create(), PartPose::offset(0.0, 1.0, -5.0));
    let neck = mesh.add_or_replace_child(skullRot, "neck", CubeListBuilder::create().tex_offs(8, 19).add_box_with([-3.0, -4.0, -6.0], [6.0, 2.0, 6.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, 0.0, 0.0));
    let neck_r1 = mesh.add_or_replace_child(neck, "neck_r1", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-2.0, -0.5, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.02)).set_mirror(false), PartPose::offset_and_rotation(-3.0, -1.5, -1.0, 0.0, 0.7854, 0.0));
    let neck_r2 = mesh.add_or_replace_child(neck, "neck_r2", CubeListBuilder::create().tex_offs(0, 0).add_box_with([0.0, -0.5, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.02)), PartPose::offset_and_rotation(3.0, -1.5, -1.0, 0.0, -0.7854, 0.0));
    let sleep = mesh.add_or_replace_child(neck, "sleep", CubeListBuilder::create().tex_offs(8, 52).add_box_with([-3.025, -5.0, -10.0], [0.0, 1.0, 2.0], CubeDeformation::new(0.0)).tex_offs(8, 52).set_mirror(true).add_box_with([3.025, -5.0, -10.0], [0.0, 1.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 2.0, 5.0));
    let lower_jaw = mesh.add_or_replace_child(neck, "lower_jaw", CubeListBuilder::create().tex_offs(33, 54).add_box_with([-3.0, 0.0, -4.0], [6.0, 2.0, 6.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, -2.0, -2.0));
    let beardie_head = mesh.add_or_replace_child(neck, "beardie_head", CubeListBuilder::create().tex_offs(49, 1).set_mirror(true).add_box_with([-3.0, -11.0, -11.0], [1.0, 1.0, 3.0], CubeDeformation::new(0.02)).set_mirror(false).tex_offs(49, 1).add_box_with([2.0, -11.0, -11.0], [1.0, 1.0, 3.0], CubeDeformation::new(0.02)).tex_offs(8, 31).add_box_with([-5.0, -12.0, -7.0], [10.0, 8.0, 0.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, 6.0, 5.0));
    let gecko = mesh.add_or_replace_child(neck, "gecko", CubeListBuilder::create().tex_offs(22, 3).add_box_with([2.0, -11.0, -10.0], [2.0, 3.0, 3.0], CubeDeformation::new(0.02)).tex_offs(22, 3).set_mirror(true).add_box_with([-4.0, -11.0, -10.0], [2.0, 3.0, 3.0], CubeDeformation::new(0.02)).set_mirror(false), PartPose::offset(0.0, 6.0, 5.0));
    let sleep_gecko = mesh.add_or_replace_child(gecko, "sleep_gecko", CubeListBuilder::create().tex_offs(0, 58).set_mirror(true).add_box_with([-4.0, -7.0, -10.0], [2.0, 3.0, 3.0], CubeDeformation::new(0.04)).set_mirror(false).tex_offs(0, 58).add_box_with([2.0, -7.0, -10.0], [2.0, 3.0, 3.0], CubeDeformation::new(0.04)), PartPose::offset(0.0, -4.0, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(28, 35).add_box_with([-1.5, -1.0, 0.0], [3.0, 3.0, 10.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 1.0, 5.0));
    let basilisk_tail = mesh.add_or_replace_child(tail, "basilisk_tail", CubeListBuilder::create().tex_offs(32, 5).add_box_with([0.0, -10.0, 5.0], [0.0, 4.0, 10.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 6.0, -5.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(0.0, 5.0, 0.0));
    let front_legs = mesh.add_or_replace_child(legs, "front_legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let left_arm = mesh.add_or_replace_child(front_legs, "left_arm", CubeListBuilder::create().tex_offs(0, 19).add_box_with([0.0, 0.0, -1.5], [5.0, 0.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(3.0, -4.0, -2.5, 0.0894, 0.3829, 0.6282));
    let right_arm = mesh.add_or_replace_child(front_legs, "right_arm", CubeListBuilder::create().tex_offs(0, 19).set_mirror(true).add_box_with([-5.0, 0.0, -1.5], [5.0, 0.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-3.0, -4.0, -2.5, 0.0894, -0.3829, -0.6282));
    let back_legs = mesh.add_or_replace_child(legs, "back_legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let left_leg = mesh.add_or_replace_child(back_legs, "left_leg", CubeListBuilder::create().tex_offs(0, 19).add_box_with([0.0, 0.0, -1.5], [5.0, 0.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(3.0, -4.0, 2.5, -0.0894, -0.3829, 0.6282));
    let right_leg = mesh.add_or_replace_child(back_legs, "right_leg", CubeListBuilder::create().tex_offs(0, 19).set_mirror(true).add_box_with([-5.0, 0.0, -1.5], [5.0, 0.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-3.0, -4.0, 2.5, -0.0894, 0.3829, -0.6282));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
