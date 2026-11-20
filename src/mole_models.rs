// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-4.0, -3.0, -4.5], [8.0, 6.0, 9.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -3.0, 1.5));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create().tex_offs(15, 19).add_box_with([-0.5, -0.5, 0.0], [1.0, 1.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 2.5, 4.5));
    let skull = mesh.add_or_replace_child(body, "skull", CubeListBuilder::create().tex_offs(0, 16).add_box_with([-3.0, -2.5, -3.6333], [6.0, 4.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 1.5, -4.8667));
    let nose = mesh.add_or_replace_child(skull, "nose", CubeListBuilder::create().tex_offs(26, 5).add_box_with([-2.5, -2.0, -2.1], [5.0, 3.0, 0.0], CubeDeformation::new(0.0)).tex_offs(26, 0).add_box_with([-1.5, -1.0, -2.0], [3.0, 2.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.5, -3.6333));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(4.0, -2.5, -2.5));
    let leftArm = mesh.add_or_replace_child(legs, "leftArm", CubeListBuilder::create().tex_offs(24, 16).add_box_with([0.0, -2.5, -0.5], [6.0, 5.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let rightArm = mesh.add_or_replace_child(legs, "rightArm", CubeListBuilder::create().tex_offs(24, 16).set_mirror(true).add_box_with([-6.0, -2.5, -0.5], [6.0, 5.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-8.0, 0.0, 0.0));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create().tex_offs(0, 25).add_box_with([0.0, -1.5, -0.5], [4.0, 3.0, 2.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 1.0, 8.0));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create().tex_offs(0, 25).set_mirror(true).add_box_with([-4.0, -1.5, -0.5], [4.0, 3.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-8.0, 1.0, 8.0));
    let mound = mesh.add_or_replace_child(partdefinition, "mound", CubeListBuilder::create().tex_offs(4, 28).add_box_with([-5.0, -3.0, -5.0], [10.0, 3.0, 10.0], CubeDeformation::new(0.0)).tex_offs(45, 38).add_box_with([4.0, -1.9, -1.0], [3.0, 2.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 24.0, 0.0));
    let mound_r1 = mesh.add_or_replace_child(mound, "mound_r1", CubeListBuilder::create().tex_offs(45, 38).add_box_with([-1.5, -1.0, -1.5], [3.0, 2.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.5, -0.9, 5.5, 0.0, -0.3927, 0.0));
    let mound_r2 = mesh.add_or_replace_child(mound, "mound_r2", CubeListBuilder::create().tex_offs(45, 38).add_box_with([-1.5, -1.0, -1.5], [3.0, 2.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(1.5, -0.9, -5.5, 0.0, 0.7854, 0.0));
    let mound_r3 = mesh.add_or_replace_child(mound, "mound_r3", CubeListBuilder::create().tex_offs(45, 38).add_box_with([-1.5, -1.0, -1.5], [3.0, 2.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-5.0, -0.9, 1.5, 0.0, 0.7854, 0.0));
    let mound_r4 = mesh.add_or_replace_child(mound, "mound_r4", CubeListBuilder::create().tex_offs(35, 29).add_box_with([-2.5, -2.0, -2.0], [5.0, 4.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-3.5, -1.9, -4.0, 0.0, -1.1781, 0.0));
    let mound_r5 = mesh.add_or_replace_child(mound, "mound_r5", CubeListBuilder::create().tex_offs(35, 29).add_box_with([-2.5, -2.0, -2.0], [5.0, 4.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(3.5, -1.9, 4.0, 0.6155, 0.5236, 0.9553));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
