// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 17.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create(), PartPose::offset(0.0, -2.0, 0.0));
    let body_r1 = mesh.add_or_replace_child(body, "body_r1", CubeListBuilder::create().tex_offs(60, 13).add_box_with([-8.0, -11.0, -19.0], [16.0, 16.0, 9.0], CubeDeformation::new(0.5)).tex_offs(0, 0).add_box_with([-8.0, -11.0, -19.0], [16.0, 16.0, 9.0], CubeDeformation::new(0.0)).tex_offs(0, 26).add_box_with([-5.5, -10.0, -22.0], [11.0, 14.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -15.0, 3.0, 1.5708, 0.0, 0.0));
    let skullRot = mesh.add_or_replace_child(body, "skullRot", CubeListBuilder::create(), PartPose::offset(0.0, 3.0, -8.0));
    let neck = mesh.add_or_replace_child(skullRot, "neck", CubeListBuilder::create().tex_offs(29, 26).add_box_with([-3.0, -3.0, -6.0], [6.0, 5.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let mask = mesh.add_or_replace_child(neck, "mask", CubeListBuilder::create().tex_offs(28, 39).add_box_with([-3.0, -7.0, -14.0], [6.0, 5.0, 7.0], CubeDeformation::new(0.5)), PartPose::offset(0.0, 4.0, 8.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(0.0, 7.0, 0.0));
    let front_legs = mesh.add_or_replace_child(legs, "front_legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let left_arm = mesh.add_or_replace_child(front_legs, "left_arm", CubeListBuilder::create().tex_offs(51, 0).add_box_with([-2.0, -2.0, -3.0], [5.0, 7.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(8.0, -5.0, -8.0, 0.0, -0.3927, 0.0));
    let right_arm = mesh.add_or_replace_child(front_legs, "right_arm", CubeListBuilder::create().tex_offs(51, 0).set_mirror(true).add_box_with([-3.0, -2.0, -3.0], [5.0, 7.0, 5.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-8.0, -5.0, -8.0, 0.0, 0.3927, 0.0));
    let back_legs = mesh.add_or_replace_child(legs, "back_legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let left_leg = mesh.add_or_replace_child(back_legs, "left_leg", CubeListBuilder::create().tex_offs(51, 0).add_box_with([-2.0, -2.0, -3.0], [5.0, 7.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(7.0, -5.0, 8.0, 0.0, -0.3927, 0.0));
    let right_leg = mesh.add_or_replace_child(back_legs, "right_leg", CubeListBuilder::create().tex_offs(51, 0).set_mirror(true).add_box_with([-3.0, -2.0, -3.0], [5.0, 7.0, 5.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-7.0, -5.0, 8.0, 0.0, 0.3927, 0.0));
    LayerDefinition::create(mesh, 128, 128).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create(), PartPose::offset(0.0, -5.0, 0.0));
    let body_r1 = mesh.add_or_replace_child(body, "body_r1", CubeListBuilder::create().tex_offs(10, 21).add_box_with([-3.5, -11.0, -22.5], [7.0, 7.0, 4.0], CubeDeformation::new(0.5)).tex_offs(0, 0).add_box_with([-3.5, -11.0, -22.5], [7.0, 7.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -20.5, 8.0, 1.5708, 0.0, 0.0));
    let body_r2 = mesh.add_or_replace_child(body, "body_r2", CubeListBuilder::create().tex_offs(0, 11).add_box_with([-3.5, -11.0, -22.5], [7.0, 7.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -18.5, 8.0, 1.5708, 0.0, 0.0));
    let skullRot = mesh.add_or_replace_child(body, "skullRot", CubeListBuilder::create(), PartPose::offset(0.0, 2.0, -2.0));
    let neck = mesh.add_or_replace_child(skullRot, "neck", CubeListBuilder::create().tex_offs(18, 11).add_box_with([-1.5, -3.0, -4.0], [3.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 1.0, 0.0));
    let asleep = mesh.add_or_replace_child(neck, "asleep", CubeListBuilder::create().tex_offs(30, 11).add_box_with([-1.5, -5.0, -6.0], [3.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 2.0, 2.0));
    let mask = mesh.add_or_replace_child(neck, "mask", CubeListBuilder::create().tex_offs(30, 5).add_box_with([-1.5, -7.0, -12.0], [3.0, 3.0, 3.0], CubeDeformation::new(0.25)), PartPose::offset(0.0, 4.0, 8.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(0.0, -1.25, 0.5381));
    let front_legs = mesh.add_or_replace_child(legs, "front_legs", CubeListBuilder::create(), PartPose::offset(0.0, 1.25, 3.7119));
    let left_arm = mesh.add_or_replace_child(front_legs, "left_arm", CubeListBuilder::create().tex_offs(22, 0).add_box_with([-1.0, -0.5, -1.0], [2.0, 3.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(3.5, -2.5, -7.5, 0.0, -0.3927, 0.0));
    let right_arm = mesh.add_or_replace_child(front_legs, "right_arm", CubeListBuilder::create().tex_offs(22, 0).set_mirror(true).add_box_with([-1.0, -0.5, -1.0], [2.0, 3.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-3.5, -2.5, -7.5, 0.0, 0.3927, 0.0));
    let back_legs = mesh.add_or_replace_child(legs, "back_legs", CubeListBuilder::create(), PartPose::offset(0.0, -1.25, 3.5381));
    let left_leg = mesh.add_or_replace_child(back_legs, "left_leg", CubeListBuilder::create().tex_offs(22, 0).add_box_with([-1.0, -0.5, -1.0], [2.0, 3.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(3.3827, 0.0, 0.0, 0.0, -0.3927, 0.0));
    let right_leg = mesh.add_or_replace_child(back_legs, "right_leg", CubeListBuilder::create().tex_offs(22, 0).set_mirror(true).add_box_with([-1.0, -0.5, -1.0], [2.0, 3.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-3.3827, 0.0, 0.0, 0.0, 0.3927, 0.0));
    LayerDefinition::create(mesh, 48, 48).bake_root()
}
