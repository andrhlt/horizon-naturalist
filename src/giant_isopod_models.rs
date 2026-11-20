// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 24).add_box_with([-5.5, -6.0, -10.0], [11.0, 6.0, 10.0], CubeDeformation::new(0.02)), PartPose::offset(0.0, -2.0, 2.0));
    let body_r1 = mesh.add_or_replace_child(body, "body_r1", CubeListBuilder::create().tex_offs(32, 32).add_box_with([-4.0, 0.0, -8.75], [4.0, 0.0, 10.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-5.5, 0.0, -1.25, 0.0, 0.0, -0.3927));
    let body_r2 = mesh.add_or_replace_child(body, "body_r2", CubeListBuilder::create().tex_offs(32, 32).set_mirror(true).add_box_with([0.0, 0.0, -8.75], [4.0, 0.0, 10.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(5.5, 0.0, -1.25, 0.0, 0.0, 0.3927));
    let body2 = mesh.add_or_replace_child(body, "body2", CubeListBuilder::create().tex_offs(34, 0).add_box_with([-5.5, -3.0, -0.5], [11.0, 6.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -3.0, 0.5));
    let body2_r1 = mesh.add_or_replace_child(body2, "body2_r1", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-3.0, 0.0, 1.25], [3.0, 0.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(-5.5, 3.0, -1.75, 0.0, 0.0, -0.3927));
    let body2_r2 = mesh.add_or_replace_child(body2, "body2_r2", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([0.0, 0.0, 1.25], [3.0, 0.0, 5.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(5.5, 3.0, -1.75, 0.0, 0.0, 0.3927));
    let tail = mesh.add_or_replace_child(body2, "tail", CubeListBuilder::create().tex_offs(24, 42).add_box_with([-4.5, -2.0, 0.0], [9.0, 4.0, 3.0], CubeDeformation::new(0.0)).tex_offs(24, 24).add_box_with([-5.5, 2.0, 0.0], [11.0, 0.0, 8.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 1.0, 4.5));
    let tailLegs = mesh.add_or_replace_child(tail, "tailLegs", CubeListBuilder::create(), PartPose::offset(2.5, 2.0, 0.0));
    let leftPleopod = mesh.add_or_replace_child(tailLegs, "leftPleopod", CubeListBuilder::create().tex_offs(0, 7).add_box_with([-2.0, 0.0, 0.0], [6.0, 5.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 1.1781, 0.0, 0.0));
    let rightPleopod = mesh.add_or_replace_child(tailLegs, "rightPleopod", CubeListBuilder::create().tex_offs(0, 7).set_mirror(true).add_box_with([-4.0, 0.0, 0.0], [6.0, 5.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-5.0, 0.0, 0.0, 1.1781, 0.0, 0.0));
    let skull = mesh.add_or_replace_child(body, "skull", CubeListBuilder::create().tex_offs(0, 40).add_box_with([-4.5, -2.0, -3.0], [9.0, 4.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -2.0, -10.0));
    let antennae = mesh.add_or_replace_child(skull, "antennae", CubeListBuilder::create(), PartPose::offset_and_rotation(0.0, 2.0, -3.0, 0.3054, 0.0, 0.0));
    let leftAntennae = mesh.add_or_replace_child(antennae, "leftAntennae", CubeListBuilder::create().tex_offs(39, 11).add_box_with([-1.5, 0.0, -7.0], [5.0, 0.0, 7.0], CubeDeformation::new(0.0)), PartPose::offset(1.5, 0.0, 2.0));
    let rightAntennae = mesh.add_or_replace_child(antennae, "rightAntennae", CubeListBuilder::create().tex_offs(39, 11).set_mirror(true).add_box_with([-3.5, 0.0, -7.0], [5.0, 0.0, 7.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-1.5, 0.0, 2.0));
    let backLegs = mesh.add_or_replace_child(body, "backLegs", CubeListBuilder::create(), PartPose::offset(0.0, 2.0, -2.0));
    let leftBackLegs = mesh.add_or_replace_child(backLegs, "leftBackLegs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let leftLeg5 = mesh.add_or_replace_child(leftBackLegs, "leftLeg5", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(4.5, -2.0, 2.0));
    let leftLeg6 = mesh.add_or_replace_child(leftBackLegs, "leftLeg6", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(4.5, -2.0, 4.0));
    let rightBackLegs = mesh.add_or_replace_child(backLegs, "rightBackLegs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let rightLeg5 = mesh.add_or_replace_child(rightBackLegs, "rightLeg5", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-1.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-4.5, -2.0, 2.0));
    let rightLeg6 = mesh.add_or_replace_child(rightBackLegs, "rightLeg6", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-1.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-4.5, -2.0, 4.0));
    let legs = mesh.add_or_replace_child(backLegs, "legs", CubeListBuilder::create(), PartPose::offset(-1.0, 2.0, 0.0));
    let leftLegs = mesh.add_or_replace_child(legs, "leftLegs", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let leftLeg1 = mesh.add_or_replace_child(leftLegs, "leftLeg1", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(5.5, -4.0, -7.0));
    let leftLeg2 = mesh.add_or_replace_child(leftLegs, "leftLeg2", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(5.5, -4.0, -5.0));
    let leftLeg3 = mesh.add_or_replace_child(leftLegs, "leftLeg3", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(5.5, -4.0, -3.0));
    let leftLeg4 = mesh.add_or_replace_child(leftLegs, "leftLeg4", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset(5.5, -4.0, -0.5));
    let rightLegs = mesh.add_or_replace_child(legs, "rightLegs", CubeListBuilder::create(), PartPose::offset(2.0, 0.0, 0.0));
    let rightLeg1 = mesh.add_or_replace_child(rightLegs, "rightLeg1", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-1.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-5.5, -4.0, -7.0));
    let rightLeg2 = mesh.add_or_replace_child(rightLegs, "rightLeg2", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-1.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-5.5, -4.0, -5.0));
    let rightLeg3 = mesh.add_or_replace_child(rightLegs, "rightLeg3", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-1.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-5.5, -4.0, -3.0));
    let rightLeg4 = mesh.add_or_replace_child(rightLegs, "rightLeg4", CubeListBuilder::create().tex_offs(0, 0).set_mirror(true).add_box_with([-1.0, 0.0, 0.0], [2.0, 2.0, 0.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-5.5, -4.0, -0.5));
    let rolled = mesh.add_or_replace_child(root, "rolled", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-5.5, -14.0, -4.0], [11.0, 12.0, 12.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 2.0, -2.0));
    let antennae2 = mesh.add_or_replace_child(rolled, "antennae2", CubeListBuilder::create(), PartPose::offset(0.0, -2.0, -4.0));
    let leftAntennae2 = mesh.add_or_replace_child(antennae2, "leftAntennae2", CubeListBuilder::create().tex_offs(41, 13).add_box_with([-1.5, 0.0, -7.0], [5.0, 0.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(1.5, 0.0, 2.0));
    let rightAntennae2 = mesh.add_or_replace_child(antennae2, "rightAntennae2", CubeListBuilder::create().tex_offs(41, 13).set_mirror(true).add_box_with([-3.5, 0.0, -7.0], [5.0, 0.0, 5.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset(-1.5, 0.0, 2.0));
    LayerDefinition::create(mesh, 80, 64).bake_root()
}
