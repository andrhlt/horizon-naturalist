// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(8, 66).add_box_with([-11.0, -9.5, -16.0], [22.0, 19.0, 34.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -18.5, 7.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(111, 63).add_box_with([-2.0, 0.0, -1.0], [4.0, 7.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -6.5, 18.0, 0.1745, 0.0, 0.0));
    let bone = mesh.add_or_replace_child(body, "bone", CubeListBuilder::create(), PartPose::offset(0.0, 0.5, -17.0));
    let neck = mesh.add_or_replace_child(bone, "neck", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let topJaw = mesh.add_or_replace_child(neck, "topJaw", CubeListBuilder::create().tex_offs(76, 23).add_box_with([-6.5, -6.0, -19.0], [13.0, 6.0, 10.0], CubeDeformation::new(0.0)).tex_offs(0, 0).add_box_with([-6.5, -7.0, -9.0], [13.0, 7.0, 9.0], CubeDeformation::new(0.0)).tex_offs(35, 0).add_box_with([2.5, -8.0, -9.0], [4.0, 1.0, 4.0], CubeDeformation::new(0.0)).tex_offs(35, 31).add_box_with([-6.5, -8.0, -9.0], [4.0, 1.0, 4.0], CubeDeformation::new(0.0)).tex_offs(0, 0).add_box_with([-7.5, -9.0, -4.0], [2.0, 2.0, 1.0], CubeDeformation::new(0.0)).tex_offs(0, 4).add_box_with([5.5, -9.0, -4.0], [2.0, 2.0, 1.0], CubeDeformation::new(0.0)).tex_offs(68, 49).add_box_with([3.5, 0.0, -15.0], [2.0, 1.0, 2.0], CubeDeformation::new(0.0)).tex_offs(68, 49).set_mirror(true).add_box_with([-5.5, 0.0, -15.0], [2.0, 1.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(39, 11).add_box_with([-6.5, 0.0, -5.0], [13.0, 5.0, 5.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, 0.0, 1.0));
    let botJaw = mesh.add_or_replace_child(neck, "botJaw", CubeListBuilder::create().tex_offs(76, 44).add_box_with([-6.5, 0.0, -14.0], [13.0, 5.0, 10.0], CubeDeformation::new(0.0)).tex_offs(77, 46).add_box_with([2.5, -5.0, -13.0], [2.0, 5.0, 2.0], CubeDeformation::new(0.0)).tex_offs(77, 46).set_mirror(true).add_box_with([-4.5, -5.0, -13.0], [2.0, 5.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(39, 21).add_box_with([-6.5, 0.0, -4.0], [13.0, 5.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, -4.0));
    let skinFlap = mesh.add_or_replace_child(botJaw, "skinFlap", CubeListBuilder::create().tex_offs(82, 7).add_box_with([-6.5, -4.5, -2.0], [13.0, 7.0, 4.0], CubeDeformation::new(-0.01)), PartPose::offset_and_rotation(0.0, -1.5, -1.0, -0.3927, 0.0, 0.0));
    let left_arm = mesh.add_or_replace_child(root, "left_arm", CubeListBuilder::create().tex_offs(3, 68).add_box_with([-3.0, -1.0, -3.5], [6.0, 10.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(7.0, -9.0, -3.5));
    let right_arm = mesh.add_or_replace_child(root, "right_arm", CubeListBuilder::create().tex_offs(3, 68).set_mirror(true).add_box_with([-3.0, -1.0, -3.5], [6.0, 10.0, 7.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-7.0, -9.0, -3.5));
    let left_leg = mesh.add_or_replace_child(root, "left_leg", CubeListBuilder::create().tex_offs(3, 68).add_box_with([-3.0, -1.0, -3.5], [6.0, 10.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(7.0, -9.0, 20.5));
    let right_leg = mesh.add_or_replace_child(root, "right_leg", CubeListBuilder::create().tex_offs(3, 68).set_mirror(true).add_box_with([-3.0, -1.0, -3.5], [6.0, 10.0, 7.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-7.0, -9.0, 19.5));
    LayerDefinition::create(mesh, 128, 128).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-5.0, -9.0, -2.0], [10.0, 9.0, 12.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -3.0, -4.0));
    let neck = mesh.add_or_replace_child(body, "neck", CubeListBuilder::create().tex_offs(28, 21).add_box_with([-4.0, -3.0, -3.5], [8.0, 2.0, 3.0], CubeDeformation::new(0.0)).tex_offs(0, 21).add_box_with([-4.0, -1.0, -6.5], [8.0, 4.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -5.0, -1.5));
    let leftEar = mesh.add_or_replace_child(neck, "leftEar", CubeListBuilder::create().tex_offs(0, 31).add_box_with([0.0, -0.5, 0.0], [1.0, 1.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(4.0, -1.5, -1.5));
    let rightEar = mesh.add_or_replace_child(neck, "rightEar", CubeListBuilder::create().tex_offs(0, 31).set_mirror(true).add_box_with([-1.0, -0.5, 0.0], [1.0, 1.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-4.0, -1.5, -1.5));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(3.0, -3.5, 3.5));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create().tex_offs(28, 26).add_box_with([-1.5, -0.5, -1.5], [3.0, 4.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create().tex_offs(28, 26).set_mirror(true).add_box_with([-1.5, -0.5, -1.5], [3.0, 4.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-6.0, 0.0, 0.0));
    let rightArm = mesh.add_or_replace_child(legs, "rightArm", CubeListBuilder::create().tex_offs(28, 26).set_mirror(true).add_box_with([-1.5, -0.5, -1.5], [3.0, 4.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-6.0, 0.0, -7.0));
    let leftArm = mesh.add_or_replace_child(legs, "leftArm", CubeListBuilder::create().tex_offs(28, 26).add_box_with([-1.5, -0.5, -1.5], [3.0, 4.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, -7.0));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
