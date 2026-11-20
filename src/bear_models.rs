// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(-1.0, 24.0, 0.0));
    let butt = mesh.add_or_replace_child(root, "butt", CubeListBuilder::create(), PartPose::offset(1.0, -15.0, 11.0));
    let body = mesh.add_or_replace_child(butt, "body", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, -12.0));
    let body_r1 = mesh.add_or_replace_child(body, "body_r1", CubeListBuilder::create().tex_offs(64, 99).add_box_with([-8.5, -13.0, -6.5], [17.0, 1.0, 15.0], CubeDeformation::new(0.3)).tex_offs(0, 0).add_box_with([-8.5, -13.0, -6.5], [17.0, 10.0, 15.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -0.5, 1.0, 1.5708, 0.0, 0.0));
    let bell = mesh.add_or_replace_child(body, "bell", CubeListBuilder::create(), PartPose::offset(0.0, 6.0, -12.5));
    let bell_r1 = mesh.add_or_replace_child(bell, "bell_r1", CubeListBuilder::create().tex_offs(116, 118).add_box_with([-1.5, -15.0, -5.5], [3.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -2.5, 12.5, 1.5708, 0.0, 0.0));
    let normalBody = mesh.add_or_replace_child(body, "normalBody", CubeListBuilder::create(), PartPose::offset(0.0, -0.5, 1.0));
    let normalBody_r1 = mesh.add_or_replace_child(normalBody, "normalBody_r1", CubeListBuilder::create().tex_offs(0, 25).add_box_with([-7.5, -3.0, -6.5], [15.0, 14.0, 13.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 1.5708, 0.0, 0.0));
    let shearedBody = mesh.add_or_replace_child(body, "shearedBody", CubeListBuilder::create(), PartPose::offset(0.0, -0.5, 1.0));
    let shearedBody_r1 = mesh.add_or_replace_child(shearedBody, "shearedBody_r1", CubeListBuilder::create().tex_offs(1, 68).add_box_with([-7.5, -3.0, -6.5], [15.0, 14.0, 13.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 1.5708, 0.0, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create(), PartPose::offset(0.0, -5.0, 12.0));
    let tail_r1 = mesh.add_or_replace_child(tail, "tail_r1", CubeListBuilder::create().tex_offs(43, 30).add_box_with([-1.5, -3.0, 2.5], [3.0, 3.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 4.5, 3.0, 1.5708, 0.0, 0.0));
    let skullRot = mesh.add_or_replace_child(body, "skullRot", CubeListBuilder::create(), PartPose::offset(0.0, 2.75, -12.375));
    let neck = mesh.add_or_replace_child(skullRot, "neck", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let normalSnout = mesh.add_or_replace_child(neck, "normalSnout", CubeListBuilder::create().tex_offs(69, 0).add_box_with([-3.5, -15.0, -24.0], [7.0, 6.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 12.25, 13.375));
    let angrySnout = mesh.add_or_replace_child(neck, "angrySnout", CubeListBuilder::create().tex_offs(69, 11).add_box_with([-3.5, -15.0, -24.0], [7.0, 6.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 12.25, 13.375));
    let nose = mesh.add_or_replace_child(neck, "nose", CubeListBuilder::create().tex_offs(49, 0).add_box_with([-3.5, -2.0, -1.0], [7.0, 2.0, 2.0], CubeDeformation::new(0.01)), PartPose::offset(0.0, -0.75, -9.625));
    let awake = mesh.add_or_replace_child(neck, "awake", CubeListBuilder::create().tex_offs(92, 0).add_box_with([-5.5, -19.0, -20.0], [11.0, 10.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 12.25, 13.375));
    let angry = mesh.add_or_replace_child(neck, "angry", CubeListBuilder::create().tex_offs(92, 36).add_box_with([-5.5, -19.0, -20.0], [11.0, 10.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 12.25, 13.375));
    let asleep = mesh.add_or_replace_child(neck, "asleep", CubeListBuilder::create().tex_offs(92, 18).add_box_with([-5.5, -19.0, -20.0], [11.0, 10.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 12.25, 13.375));
    let leftEar = mesh.add_or_replace_child(neck, "leftEar", CubeListBuilder::create().tex_offs(90, 0).add_box_with([-1.5, -2.5, -0.5], [3.0, 3.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(5.0, -6.25, -4.125));
    let rightEar = mesh.add_or_replace_child(neck, "rightEar", CubeListBuilder::create().tex_offs(90, 0).set_mirror(true).add_box_with([-1.5, -2.5, -0.5], [3.0, 3.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-5.0, -6.25, -4.125));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(-3.0, -9.0, -9.0));
    let rightArm = mesh.add_or_replace_child(legs, "rightArm", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let normalArm = mesh.add_or_replace_child(rightArm, "normalArm", CubeListBuilder::create().tex_offs(50, 49).set_mirror(true).add_box_with([-6.5, -9.0, -12.0], [5.0, 9.0, 6.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(4.0, 9.0, 9.0));
    let saucyArm = mesh.add_or_replace_child(rightArm, "saucyArm", CubeListBuilder::create().tex_offs(73, 49).set_mirror(true).add_box_with([-6.5, -9.0, -12.0], [5.0, 9.0, 6.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(4.0, 9.0, 9.0));
    let honeyArm = mesh.add_or_replace_child(saucyArm, "honeyArm", CubeListBuilder::create().tex_offs(27, 52).set_mirror(true).add_box_with([-6.5, -6.0, -12.0], [5.0, 6.0, 6.0], CubeDeformation::new(0.25)).set_mirror(false), PartPose::offset(0.0, 0.0, 0.0));
    let berryArm = mesh.add_or_replace_child(saucyArm, "berryArm", CubeListBuilder::create().tex_offs(4, 52).set_mirror(true).add_box_with([-6.5, -6.0, -12.0], [5.0, 6.0, 6.0], CubeDeformation::new(0.25)).set_mirror(false), PartPose::offset(0.0, 0.0, 0.0));
    let rightHand = mesh.add_or_replace_child(rightArm, "rightHand", CubeListBuilder::create(), PartPose::offset(-0.5, 7.0, -3.0));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create().tex_offs(50, 49).add_box_with([-2.5, 0.0, -3.0], [5.0, 9.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(8.0, 0.0, 16.0));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create().tex_offs(50, 49).set_mirror(true).add_box_with([-2.5, 0.0, -3.0], [5.0, 9.0, 6.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(0.0, 0.0, 16.0));
    let leftArm = mesh.add_or_replace_child(legs, "leftArm", CubeListBuilder::create().tex_offs(50, 49).add_box_with([-2.5, 0.0, -3.0], [5.0, 9.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(8.0, 0.0, 0.0));
    LayerDefinition::create(mesh, 128, 128).bake_root()
}
pub(super) fn baby() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(-1.0, 24.0, -1.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-6.5, -5.5, -7.25], [13.0, 10.0, 14.0], CubeDeformation::new(0.0)), PartPose::offset(1.0, -9.5, 4.25));
    let body_r1 = mesh.add_or_replace_child(body, "body_r1", CubeListBuilder::create().tex_offs(82, 117).add_box_with([-6.5, -13.0, -3.5], [13.0, 1.0, 10.0], CubeDeformation::new(0.3)), PartPose::offset_and_rotation(0.0, 1.0, 5.75, 1.5708, 0.0, 0.0));
    let bell = mesh.add_or_replace_child(body, "bell", CubeListBuilder::create(), PartPose::offset(0.0, 3.5, -7.75));
    let bell_r1 = mesh.add_or_replace_child(bell, "bell_r1", CubeListBuilder::create().tex_offs(116, 118).add_box_with([-1.5, -15.0, -5.5], [3.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, -2.5, 12.5, 1.5708, 0.0, 0.0));
    let tail = mesh.add_or_replace_child(body, "tail", CubeListBuilder::create(), PartPose::offset(0.0, -3.5, 6.75));
    let tail_r1 = mesh.add_or_replace_child(tail, "tail_r1", CubeListBuilder::create().tex_offs(36, 34).add_box_with([-1.5, -3.0, 2.5], [3.0, 3.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 4.5, 3.0, 1.5708, 0.0, 0.0));
    let skullRot = mesh.add_or_replace_child(body, "skullRot", CubeListBuilder::create(), PartPose::offset(0.0, 0.25, -7.625));
    let neck = mesh.add_or_replace_child(skullRot, "neck", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let leftEar = mesh.add_or_replace_child(neck, "leftEar", CubeListBuilder::create().tex_offs(12, 39).add_box_with([-1.5, -2.0, -0.5], [3.0, 2.0, 1.0], CubeDeformation::new(0.0)), PartPose::offset(4.0, -4.75, -4.125));
    let rightEar = mesh.add_or_replace_child(neck, "rightEar", CubeListBuilder::create().tex_offs(12, 39).set_mirror(true).add_box_with([-1.5, -3.0, -0.5], [3.0, 2.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-4.0, -3.75, -4.125));
    let awake = mesh.add_or_replace_child(neck, "awake", CubeListBuilder::create().tex_offs(0, 24).add_box_with([-5.5, -4.0, -3.5], [11.0, 8.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -0.75, -3.125));
    let asleep = mesh.add_or_replace_child(neck, "asleep", CubeListBuilder::create().tex_offs(19, 43).add_box_with([-5.5, -16.5, -18.9], [11.0, 8.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 11.75, 12.275));
    let snout = mesh.add_or_replace_child(neck, "snout", CubeListBuilder::create().tex_offs(0, 39).add_box_with([-1.5, -2.0, -3.0], [3.0, 4.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 1.25, -6.625));
    let nose = mesh.add_or_replace_child(snout, "nose", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.5, -2.0, -1.0], [3.0, 2.0, 2.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, 0.0, -2.0));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(4.5, -5.0, -0.5));
    let leftArm = mesh.add_or_replace_child(legs, "leftArm", CubeListBuilder::create().tex_offs(36, 24).add_box_with([-2.0, 0.0, -2.5], [4.0, 5.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let rightArm = mesh.add_or_replace_child(legs, "rightArm", CubeListBuilder::create().tex_offs(36, 24).set_mirror(true).add_box_with([-2.0, 0.0, -2.5], [4.0, 5.0, 5.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-7.0, 0.0, 0.0));
    let rightLeg = mesh.add_or_replace_child(legs, "rightLeg", CubeListBuilder::create().tex_offs(36, 24).set_mirror(true).add_box_with([-2.0, 0.0, -3.0], [4.0, 5.0, 5.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-7.0, 0.0, 8.5));
    let leftLeg = mesh.add_or_replace_child(legs, "leftLeg", CubeListBuilder::create().tex_offs(36, 24).add_box_with([-2.0, 0.0, -3.0], [4.0, 5.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 8.5));
    LayerDefinition::create(mesh, 128, 128).bake_root()
}
