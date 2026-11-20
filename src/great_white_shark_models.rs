// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-10.0, -6.5, -17.0], [20.0, 21.0, 40.0], CubeDeformation::new(0.01)), PartPose::offset(0.0, -15.5, -11.0));
    let skullRot = mesh.add_or_replace_child(body, "skullRot", CubeListBuilder::create(), PartPose::offset(0.0, 0.3333, -16.8333));
    let topJaw = mesh.add_or_replace_child(skullRot, "topJaw", CubeListBuilder::create().tex_offs(54, 77).add_box_with([-7.0, -2.0, -21.0], [14.0, 11.0, 22.0], CubeDeformation::new(0.04)).tex_offs(68, 114).add_box_with([-7.0, 1.0, -10.0], [14.0, 8.0, 11.0], CubeDeformation::new(0.0)).tex_offs(43, 138).add_box_with([-17.0, 1.0, -21.0], [34.0, 8.0, 11.0], CubeDeformation::new(0.02)).tex_offs(80, 26).add_box_with([-6.0, 8.0, -20.0], [12.0, 2.0, 10.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, -4.8333, -0.1667));
    let bottomJaw = mesh.add_or_replace_child(skullRot, "bottomJaw", CubeListBuilder::create().tex_offs(104, 75).add_box_with([-6.0, -0.1667, -12.1667], [12.0, 5.0, 9.0], CubeDeformation::new(0.0)).tex_offs(114, 1).add_box_with([-5.0, -1.1667, -11.1667], [10.0, 2.0, 11.0], CubeDeformation::new(0.0)).tex_offs(7, 106).add_box_with([-7.0, -0.1667, -3.1667], [14.0, 5.0, 3.0], CubeDeformation::new(0.02)).tex_offs(1, 132).add_box_with([-7.0, -0.0809, -0.1809], [14.0, 5.0, 7.0], CubeDeformation::new(0.03)), PartPose::offset_and_rotation(0.0, 4.2476, -5.9858, 0.0873, 0.0, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(0, 61).add_box_with([-7.0, -6.1048, -4.9323], [14.0, 14.0, 24.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, 2.6048, 22.9323));
    let bottomFin = mesh.add_or_replace_child(tail, "bottomFin", CubeListBuilder::create().tex_offs(0, 61).add_box_with([-1.0, -0.5, 0.2], [2.0, 7.0, 6.0], CubeDeformation::new(0.02)), PartPose::offset_and_rotation(0.0, 8.3952, 6.0677, 0.9599, 0.0, 0.0));
    let tail2 = mesh.add_or_replace_child(tail, "tail2", CubeListBuilder::create().tex_offs(80, 0).add_box_with([-3.0, -5.0, -2.0], [6.0, 9.0, 17.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.1048, 19.0677));
    let fluke = mesh.add_or_replace_child(tail2, "fluke", CubeListBuilder::create().tex_offs(52, 61).add_box_with([-13.0, -1.5, 0.0], [28.0, 3.0, 11.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -0.5, 13.0, 0.0, 0.0, -1.5708));
    let backFin = mesh.add_or_replace_child(body, "backFin", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.5, 14.0, -16.5], [2.0, 23.0, 12.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.5, -25.5, 22.5, -0.3491, 0.0, 0.0));
    let rightFin = mesh.add_or_replace_child(root, "rightFin", CubeListBuilder::create().tex_offs(37, 103).add_box_with([-1.0, 0.0, -5.0], [2.0, 21.0, 11.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-10.0, -7.0, -14.0, 0.0, 0.0, 1.5708));
    let leftFin = mesh.add_or_replace_child(root, "leftFin", CubeListBuilder::create().tex_offs(37, 103).set_mirror(true).add_box_with([-1.0, 0.0, -5.0], [2.0, 21.0, 11.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(10.0, -7.0, -14.0, 0.0, 0.0, -1.5708));
    LayerDefinition::create(mesh, 160, 160).bake_root()
}
