// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 22.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-17.0, -18.5, -25.0], [34.0, 36.0, 58.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -15.5, -14.0));
    let skullRot = mesh.add_or_replace_child(body, "skullRot", CubeListBuilder::create().tex_offs(184, 53).add_box_with([-14.0, -1.8333, -12.9167], [28.0, 16.0, 13.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, -5.6667, -25.0833));
    let topJaw = mesh.add_or_replace_child(skullRot, "topJaw", CubeListBuilder::create().tex_offs(126, 0).add_box_with([-14.0, -10.0, -37.0], [28.0, 11.0, 42.0], CubeDeformation::new(0.0)).tex_offs(161, 186).add_box_with([-12.0, 1.0, -35.0], [24.0, 8.0, 0.0], CubeDeformation::new(-0.02)).tex_offs(128, 134).add_box_with([-12.0, 1.0, -35.0], [0.0, 8.0, 42.0], CubeDeformation::new(-0.02)).tex_offs(128, 134).set_mirror(true).add_box_with([12.0, 1.0, -35.0], [0.0, 8.0, 42.0], CubeDeformation::new(-0.02)).set_mirror(false), PartPose::offset(0.0, -2.8333, -4.9167));
    let bottomJaw = mesh.add_or_replace_child(skullRot, "bottomJaw", CubeListBuilder::create().tex_offs(158, 227).add_box_with([-14.0, 0.0, -29.0], [28.0, 16.0, 29.0], CubeDeformation::new(0.0)).tex_offs(0, 8).add_box_with([-12.0, 0.0, -27.0], [0.0, 5.0, 27.0], CubeDeformation::new(0.0)).tex_offs(0, 8).set_mirror(true).add_box_with([12.0, 0.0, -27.0], [0.0, 5.0, 27.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(0, 8).add_box_with([-12.0, 0.0, -27.0], [24.0, 5.0, 0.0], CubeDeformation::new(0.0)).tex_offs(0, 0).add_box_with([-8.0, 0.0, -27.0], [0.0, 6.0, 29.0], CubeDeformation::new(0.0)).tex_offs(0, 0).set_mirror(true).add_box_with([8.0, 0.0, -27.0], [0.0, 6.0, 29.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(96, 127).add_box_with([-12.0, 5.0, -27.0], [24.0, 0.0, 40.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.8333, -12.9167));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(3, 141).add_box_with([-13.0, -11.1048, -3.9323], [26.0, 26.0, 46.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, -2.3952, 32.9323));
    let tail2 = mesh.add_or_replace_child(tail, "tail2", CubeListBuilder::create().tex_offs(16, 220).add_box_with([-9.0, -8.0, -4.0], [18.0, 15.0, 37.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -3.1048, 42.0677));
    let fluke = mesh.add_or_replace_child(tail2, "fluke", CubeListBuilder::create().tex_offs(94, 94).add_box_with([-28.0, -2.5, -4.0], [56.0, 5.0, 28.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.5, 26.0));
    let bone = mesh.add_or_replace_child(tail, "bone", CubeListBuilder::create(), PartPose::offset(0.0, -3.6048, 19.5677));
    let bone_r1 = mesh.add_or_replace_child(bone, "bone_r1", CubeListBuilder::create().tex_offs(184, 127).add_box_with([-2.0, -15.5, -15.5], [4.0, 11.0, 21.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.3927, 0.0, 0.0));
    let rightFin = mesh.add_or_replace_child(root, "rightFin", CubeListBuilder::create().tex_offs(226, 142).add_box_with([-1.5, 0.0, -10.0], [3.0, 47.0, 20.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-17.0, -9.5, -14.0, 0.0, 0.0, 1.5708));
    let leftFin = mesh.add_or_replace_child(root, "leftFin", CubeListBuilder::create().tex_offs(226, 142).set_mirror(true).add_box_with([-1.5, 0.0, -10.0], [3.0, 47.0, 20.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(17.0, -9.5, -14.0, 0.0, 0.0, -1.5708));
    let particle = mesh.add_or_replace_child(root, "particle", CubeListBuilder::create(), PartPose::offset(0.0, -34.0, -51.0));
    LayerDefinition::create(mesh, 272, 272).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 19.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-8.0, -10.0, -14.0], [16.0, 15.0, 24.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 3.0));
    let leftFin = mesh.add_or_replace_child(body, "leftFin", CubeListBuilder::create().tex_offs(0, 79).add_box_with([1.0, -1.0, -1.0], [17.0, 2.0, 8.0], CubeDeformation::new(0.0)), PartPose::offset(7.0, 0.0, -9.0));
    let rightFin = mesh.add_or_replace_child(body, "rightFin", CubeListBuilder::create().tex_offs(0, 79).set_mirror(true).add_box_with([-18.0, -1.0, -1.0], [17.0, 2.0, 8.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-7.0, 0.0, -9.0));
    let skull = mesh.add_or_replace_child(body, "skull", CubeListBuilder::create().tex_offs(58, 50).add_box_with([-6.0, -4.0, -19.0], [12.0, 6.0, 18.0], CubeDeformation::new(0.0)).tex_offs(80, 0).add_box_with([-6.0, 2.0, -7.0], [12.0, 6.0, 6.0], CubeDeformation::new(0.026)).tex_offs(0, 89).add_box_with([-5.0, 0.0, -18.0], [10.0, 5.0, 11.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -6.0, -13.0));
    let jaw = mesh.add_or_replace_child(skull, "jaw", CubeListBuilder::create().tex_offs(58, 74).add_box_with([-6.0, 0.0, -12.0], [12.0, 6.0, 12.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 2.0, -7.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(0, 50).add_box_with([-5.0, -5.0, 1.0], [10.0, 10.0, 19.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -3.0, 8.0));
    let flip = mesh.add_or_replace_child(tail, "flip", CubeListBuilder::create().tex_offs(0, 39).add_box_with([-16.0, -1.0, -2.0], [32.0, 2.0, 9.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 19.0));
    LayerDefinition::create(mesh, 128, 128).bake_root()
}
