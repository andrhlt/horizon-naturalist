// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, -8.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-12.0, -15.5, -24.0], [24.0, 29.0, 27.0], CubeDeformation::new(0.0)).tex_offs(0, 57).add_box_with([-10.0, -11.5, 3.0], [20.0, 25.0, 15.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -27.5, 11.0));
    let attack = mesh.add_or_replace_child(body, "attack", CubeListBuilder::create(), PartPose::offset(0.0, 1.5, -24.5));
    let skullRot = mesh.add_or_replace_child(attack, "skullRot", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let neck = mesh.add_or_replace_child(skullRot, "neck", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 2.0));
    let neck_r1 = mesh.add_or_replace_child(neck, "neck_r1", CubeListBuilder::create().tex_offs(76, 0).add_box_with([-5.0, -30.0, -43.0], [10.0, 10.0, 12.0], CubeDeformation::new(0.0)).tex_offs(71, 57).add_box_with([-8.0, -38.0, -31.0], [16.0, 18.0, 12.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 20.0671, 25.5648, 0.2618, 0.0, 0.0));
    let baby_horn = mesh.add_or_replace_child(neck, "baby_horn", CubeListBuilder::create().tex_offs(1, 100).add_box_with([-2.5, -32.0, -37.0], [5.0, 5.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 18.7222, 18.9928, 0.2618, 0.0, 0.0));
    let big_horn = mesh.add_or_replace_child(neck, "big_horn", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-2.5, -32.0, -37.0], [5.0, 8.0, 5.0], CubeDeformation::new(0.02)), PartPose::offset_and_rotation(0.0, 15.8245, 18.2164, 0.2618, 0.0, 0.0));
    let small_horn = mesh.add_or_replace_child(neck, "small_horn", CubeListBuilder::create().tex_offs(0, 57).add_box_with([-1.5, -29.0, -30.0], [3.0, 5.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 15.8245, 18.2164, 0.2618, 0.0, 0.0));
    let right_ear = mesh.add_or_replace_child(neck, "right_ear", CubeListBuilder::create().tex_offs(0, 14).set_mirror(true).add_box_with([-3.5, -8.0, -1.0], [5.0, 8.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-7.5, -10.6852, -6.4866, 0.2618, 0.0, 0.0));
    let left_ear = mesh.add_or_replace_child(neck, "left_ear", CubeListBuilder::create().tex_offs(0, 14).add_box_with([-1.5, -8.0, -1.0], [5.0, 8.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(7.5, -10.6852, -6.4866, 0.2618, 0.0, 0.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let front_legs = mesh.add_or_replace_child(legs, "front_legs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let left_arm = mesh.add_or_replace_child(front_legs, "left_arm", CubeListBuilder::create().tex_offs(60, 88).add_box_with([-4.0, 0.0, -5.5], [8.0, 14.0, 11.0], CubeDeformation::new(0.0)), PartPose::offset(7.0, -14.0, -5.5));
    let right_arm = mesh.add_or_replace_child(front_legs, "right_arm", CubeListBuilder::create().tex_offs(60, 88).add_box_with([-4.0, 0.0, -5.5], [8.0, 14.0, 11.0], CubeDeformation::new(0.0)), PartPose::offset(-7.0, -14.0, -5.5));
    let back_legs = mesh.add_or_replace_child(legs, "back_legs", CubeListBuilder::create(), PartPose::offset(2.0, 0.0, 27.0));
    let left_leg = mesh.add_or_replace_child(back_legs, "left_leg", CubeListBuilder::create().tex_offs(60, 88).add_box_with([-4.0, 0.0, -5.5], [8.0, 14.0, 11.0], CubeDeformation::new(0.0)), PartPose::offset(-7.0, -14.0, -5.5));
    let right_leg = mesh.add_or_replace_child(back_legs, "right_leg", CubeListBuilder::create().tex_offs(60, 88).add_box_with([-4.0, 0.0, -5.5], [8.0, 14.0, 11.0], CubeDeformation::new(0.0)), PartPose::offset(3.0, -14.0, -5.5));
    LayerDefinition::create(mesh, 128, 128).bake_root()
}
