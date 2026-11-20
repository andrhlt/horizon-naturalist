// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-3.5, -4.0, -9.0], [7.0, 6.0, 11.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -3.0, 5.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(42, 16).add_box_with([-0.5, -0.5, 0.0], [1.0, 1.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -3.5, 2.0));
    let tail2 = mesh.add_or_replace_child(tail, "tail2", CubeListBuilder::create().tex_offs(41, 22).add_box_with([-0.5, -0.5, 0.0], [1.0, 1.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 4.0));
    let skull = mesh.add_or_replace_child(body, "skull", CubeListBuilder::create().tex_offs(4, 3).add_box_with([-1.0, 2.0, -6.0], [2.0, 1.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -3.0, -9.0));
    let awake = mesh.add_or_replace_child(skull, "awake", CubeListBuilder::create().tex_offs(0, 17).add_box_with([-2.5, -8.0, -10.0], [5.0, 4.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 6.0, 4.0));
    let sleep = mesh.add_or_replace_child(skull, "sleep", CubeListBuilder::create().tex_offs(42, 0).add_box_with([-2.5, -8.0, -10.0], [5.0, 4.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 6.0, 4.0));
    let leftWhisker = mesh.add_or_replace_child(skull, "leftWhisker", CubeListBuilder::create().tex_offs(0, 3).add_box_with([0.0, -1.5, 0.0], [2.0, 3.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(2.5, -0.5, -6.0));
    let rightWhisker = mesh.add_or_replace_child(skull, "rightWhisker", CubeListBuilder::create().tex_offs(0, 3).set_mirror(true).add_box_with([-2.0, -1.5, 0.0], [2.0, 3.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-2.5, -0.5, -6.0));
    let leftEar = mesh.add_or_replace_child(skull, "leftEar", CubeListBuilder::create().tex_offs(0, 0).add_box_with([0.0, -2.5, 0.0], [3.0, 3.0, 0.0], CubeDeformation::new(0.01)), PartPose::offset(2.5, -1.5, 0.0));
    let rightEar = mesh.add_or_replace_child(skull, "rightEar", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-3.0, -2.5, 0.0], [3.0, 3.0, 0.0], CubeDeformation::new(0.01)).set_mirror(false), PartPose::offset(-2.5, -1.5, 0.0));
    let leftLeg = mesh.add_or_replace_child(root, "leftLeg", CubeListBuilder::create().tex_offs(24, 17).add_box_with([-1.5, 0.0, -4.0], [3.0, 1.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(3.5, -1.0, 5.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(2.5, -1.0, -3.5));
    let leftArm = mesh.add_or_replace_child(legs, "leftArm", CubeListBuilder::create().tex_offs(25, 0).add_box_with([-1.5, 0.0, -2.5], [3.0, 1.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let rightArm = mesh.add_or_replace_child(legs, "rightArm", CubeListBuilder::create().tex_offs(25, 0).set_mirror(true).add_box_with([-1.5, 0.0, -2.5], [3.0, 1.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-5.0, 0.0, 0.0));
    let rightLeg = mesh.add_or_replace_child(root, "rightLeg", CubeListBuilder::create().tex_offs(24, 17).set_mirror(true).add_box_with([-1.5, 0.0, -4.0], [3.0, 1.0, 6.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-3.5, -1.0, 5.0));
    LayerDefinition::create(mesh, 64, 32).bake_root()
}
