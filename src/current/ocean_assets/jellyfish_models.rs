// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 22.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-6.0, -4.0, -6.0], [12.0, 7.0, 12.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -9.0, 0.0));
    let inflation = mesh.add_or_replace_child(body, "inflation", CubeListBuilder::create().tex_offs(0, 20).add_box_with([-6.0, -15.5, -6.0], [12.0, 7.0, 12.0], CubeDeformation::new(1.0)), PartPose::offset(0.0, 11.0, 0.0));
    let tentacles = mesh.add_or_replace_child(body, "tentacles", CubeListBuilder::create(), PartPose::offset(0.0, 3.0, 2.5));
    let frontTentacle = mesh.add_or_replace_child(tentacles, "frontTentacle", CubeListBuilder::create().tex_offs(3, 2).add_box_with([-0.5, 0.0, -0.5], [1.0, 8.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, -5.0));
    let leftTentacle = mesh.add_or_replace_child(tentacles, "leftTentacle", CubeListBuilder::create(), PartPose::offset(3.0, 0.0, -2.5));
    let leftTentacle_r1 = mesh.add_or_replace_child(leftTentacle, "leftTentacle_r1", CubeListBuilder::create().tex_offs(3, 2).add_box_with([-0.5, -4.0, -0.5], [1.0, 8.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 4.0, 0.0, 0.0, -1.5708, 0.0));
    let leftMidFrontTentacle = mesh.add_or_replace_child(tentacles, "leftMidFrontTentacle", CubeListBuilder::create(), PartPose::offset(2.0, 0.0, -4.5));
    let leftMidFrontTentacle_r1 = mesh.add_or_replace_child(leftMidFrontTentacle, "leftMidFrontTentacle_r1", CubeListBuilder::create().tex_offs(3, 2).add_box_with([-0.5, -4.0, -0.5], [1.0, 8.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 4.0, 0.0, 0.0, -1.5708, 0.0));
    let leftMidBackTentacle = mesh.add_or_replace_child(tentacles, "leftMidBackTentacle", CubeListBuilder::create(), PartPose::offset(2.0, 0.0, -0.5));
    let leftMidBackTentacle_r1 = mesh.add_or_replace_child(leftMidBackTentacle, "leftMidBackTentacle_r1", CubeListBuilder::create().tex_offs(3, 2).add_box_with([-0.5, -4.0, -0.5], [1.0, 8.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 4.0, 0.0, 0.0, -1.5708, 0.0));
    let rightMidFrontTentacle = mesh.add_or_replace_child(tentacles, "rightMidFrontTentacle", CubeListBuilder::create(), PartPose::offset(-2.0, 0.0, -4.5));
    let rightMidFrontTentacle_r1 = mesh.add_or_replace_child(rightMidFrontTentacle, "rightMidFrontTentacle_r1", CubeListBuilder::create().tex_offs(3, 2).add_box_with([-0.5, -4.0, -0.5], [1.0, 8.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 4.0, 0.0, 0.0, 1.5708, 0.0));
    let rightMidBackTentacle = mesh.add_or_replace_child(tentacles, "rightMidBackTentacle", CubeListBuilder::create(), PartPose::offset(-2.0, 0.0, -0.5));
    let rightMidBackTentacle_r1 = mesh.add_or_replace_child(rightMidBackTentacle, "rightMidBackTentacle_r1", CubeListBuilder::create().tex_offs(3, 2).add_box_with([-0.5, -4.0, -0.5], [1.0, 8.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 4.0, 0.0, 0.0, 1.5708, 0.0));
    let backTentacle = mesh.add_or_replace_child(tentacles, "backTentacle", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let backTentacle_r1 = mesh.add_or_replace_child(backTentacle, "backTentacle_r1", CubeListBuilder::create().tex_offs(3, 2).add_box_with([-0.5, -4.0, -0.5], [1.0, 8.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 4.0, 0.0, 0.0, 3.1416, 0.0));
    let rightTentacle = mesh.add_or_replace_child(tentacles, "rightTentacle", CubeListBuilder::create(), PartPose::offset(-3.0, 0.0, -2.5));
    let rightTentacle_r1 = mesh.add_or_replace_child(rightTentacle, "rightTentacle_r1", CubeListBuilder::create().tex_offs(3, 2).add_box_with([-0.5, -4.0, -0.5], [1.0, 8.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 4.0, 0.0, 0.0, 1.5708, 0.0));
    let skirt = mesh.add_or_replace_child(body, "skirt", CubeListBuilder::create(), PartPose::offset(0.0, 3.0, 4.0));
    let bone4 = mesh.add_or_replace_child(skirt, "bone4", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let bone4_r1 = mesh.add_or_replace_child(bone4, "bone4_r1", CubeListBuilder::create().tex_offs(37, 20).add_box_with([-5.0, 0.0, 0.0], [10.0, 5.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.3927, 0.0, 0.0));
    let bone3 = mesh.add_or_replace_child(skirt, "bone3", CubeListBuilder::create(), PartPose::offset(4.0, 0.0, -4.0));
    let bone3_r1 = mesh.add_or_replace_child(bone3, "bone3_r1", CubeListBuilder::create().tex_offs(37, 10).add_box_with([0.0, 0.0, -5.0], [0.0, 5.0, 10.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, 0.0, -0.3927));
    let bone2 = mesh.add_or_replace_child(skirt, "bone2", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, -8.0));
    let bone2_r1 = mesh.add_or_replace_child(bone2, "bone2_r1", CubeListBuilder::create().tex_offs(37, 20).add_box_with([-5.0, 0.0, 0.0], [10.0, 5.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, -0.3927, 0.0, 0.0));
    let bone = mesh.add_or_replace_child(skirt, "bone", CubeListBuilder::create(), PartPose::offset(-4.0, 0.0, -4.0));
    let bone_r1 = mesh.add_or_replace_child(bone, "bone_r1", CubeListBuilder::create().tex_offs(37, 10).add_box_with([0.0, 0.0, -5.0], [0.0, 5.0, 10.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, 0.0, 0.3927));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
