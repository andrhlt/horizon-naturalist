// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-5.0, -5.0, -9.0], [10.0, 10.0, 18.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -16.0, 0.0));
    let saddle = mesh.add_or_replace_child(body, "saddle", CubeListBuilder::create().tex_offs(77, 5).add_box_with([-5.0, -21.0, -3.0], [10.0, 9.0, 9.0], CubeDeformation::new(0.5)), PartPose::offset(0.0, 16.0, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(5, 11).add_box_with([-2.0, -3.0, 0.0], [4.0, 4.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -5.0, 9.0));
    let skullRot = mesh.add_or_replace_child(body, "skullRot", CubeListBuilder::create(), PartPose::offset(0.0, -1.0, -9.0));
    let neck = mesh.add_or_replace_child(skullRot, "neck", CubeListBuilder::create().tex_offs(44, 5).add_box_with([-2.0, -11.0, -7.0], [4.0, 3.0, 4.0], CubeDeformation::new(0.0)).tex_offs(41, 29).add_box_with([-3.0, -13.0, -3.0], [6.0, 15.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let sleep = mesh.add_or_replace_child(neck, "sleep", CubeListBuilder::create().tex_offs(17, 81).add_box_with([-3.0, -29.025, -12.025], [2.025, 1.025, 0.0], CubeDeformation::new(0.0)).tex_offs(15, 79).add_box_with([-3.0, -29.025, -12.025], [0.0, 1.025, 2.0], CubeDeformation::new(0.0)).tex_offs(17, 81).set_mirror(true).add_box_with([1.0, -29.025, -12.025], [2.025, 1.025, 0.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(15, 79).set_mirror(true).add_box_with([3.025, -29.025, -12.025], [0.0, 1.025, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 17.0, 9.0));
    let left_ear = mesh.add_or_replace_child(neck, "left_ear", CubeListBuilder::create().tex_offs(1, 6).add_box_with([0.0, -1.0, -0.5], [3.0, 2.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(3.0, -12.0, 0.5));
    let right_ear = mesh.add_or_replace_child(neck, "right_ear", CubeListBuilder::create().tex_offs(1, 6).set_mirror(true).add_box_with([-3.0, -1.0, -0.5], [3.0, 2.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-3.0, -12.0, 0.5));
    let antlers = mesh.add_or_replace_child(neck, "antlers", CubeListBuilder::create().tex_offs(0, 29).add_box_with([-1.0, -9.01, -4.0], [10.0, 8.0, 10.0], CubeDeformation::new(0.0)).tex_offs(0, 29).set_mirror(true).add_box_with([-13.0, -9.01, -4.0], [10.0, 8.0, 10.0], CubeDeformation::new(0.0)).set_mirror(false).tex_offs(0, 0).add_box_with([-1.0, -1.0, -2.0], [2.0, 2.0, 2.0], CubeDeformation::new(0.0)).tex_offs(0, 0).set_mirror(true).add_box_with([-5.0, -1.0, -2.0], [2.0, 2.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(2.0, -14.0, 1.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(0.0, -17.0, -6.0));
    let frontLegs = mesh.add_or_replace_child(legs, "frontLegs", CubeListBuilder::create(), PartPose::offset(0.0, 17.0, 6.0));
    let leftArm = mesh.add_or_replace_child(frontLegs, "leftArm", CubeListBuilder::create().tex_offs(1, 48).add_box_with([-1.5, 0.0, -2.0], [3.0, 14.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(3.25, -14.0, -5.0));
    let rightArm = mesh.add_or_replace_child(frontLegs, "rightArm", CubeListBuilder::create().tex_offs(1, 48).add_box_with([-1.5, 0.0, -2.0], [3.0, 14.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(-3.25, -14.0, -5.0));
    let backLegs = mesh.add_or_replace_child(legs, "backLegs", CubeListBuilder::create(), PartPose::offset(0.0, 17.0, 6.0));
    let leftLeg = mesh.add_or_replace_child(backLegs, "leftLeg", CubeListBuilder::create().tex_offs(1, 48).add_box_with([-1.5, 0.0, -2.0], [3.0, 14.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(3.25, -14.0, 9.0));
    let rightLeg = mesh.add_or_replace_child(backLegs, "rightLeg", CubeListBuilder::create().tex_offs(1, 48).add_box_with([-1.5, 0.0, -2.0], [3.0, 14.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(-3.25, -14.0, 9.0));
    LayerDefinition::create(mesh, 128, 128).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-2.0, -1.5, -4.0], [4.0, 4.0, 8.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -8.5, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(16, 12).add_box_with([-1.0, -3.5, -1.0], [2.0, 4.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 4.0));
    let neck = mesh.add_or_replace_child(body, "neck", CubeListBuilder::create().tex_offs(0, 12).add_box_with([-1.0, -6.0, -2.0], [2.0, 8.0, 3.0], CubeDeformation::new(0.0)).tex_offs(16, 18).add_box_with([-1.0, -6.0, -4.0], [2.0, 3.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -0.5, -4.0));
    let leftEar = mesh.add_or_replace_child(neck, "leftEar", CubeListBuilder::create().tex_offs(10, 20).add_box_with([0.0, -3.0, 0.0], [2.0, 3.0, 0.0], CubeDeformation::new(0.02)), PartPose::offset(1.0, -5.0, 1.0));
    let rightEar = mesh.add_or_replace_child(neck, "rightEar", CubeListBuilder::create().tex_offs(10, 20).set_mirror(true).add_box_with([-2.0, -3.0, 0.0], [2.0, 3.0, 0.0], CubeDeformation::new(0.02)).set_mirror(false), PartPose::offset(-1.0, -5.0, 1.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(-1.25, -6.0, -3.0));
    let rightArm = mesh.add_or_replace_child(legs, "rightArm", CubeListBuilder::create().tex_offs(10, 12).set_mirror(true).add_box_with([-0.75, 0.0, -1.0], [1.0, 6.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 0.0, 0.0));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create().tex_offs(10, 12).add_box_with([-0.25, 0.0, -1.0], [1.0, 6.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(2.5, 0.0, 7.0));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create().tex_offs(10, 12).set_mirror(true).add_box_with([-0.75, 0.0, -1.0], [1.0, 6.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 0.0, 7.0));
    let leftArm = mesh.add_or_replace_child(legs, "leftArm", CubeListBuilder::create().tex_offs(10, 12).add_box_with([-0.25, 0.0, -1.0], [1.0, 6.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(2.5, 0.0, 0.0));
    LayerDefinition::create(mesh, 32, 32).bake_root()
}
