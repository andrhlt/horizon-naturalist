// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let neck = mesh.add_or_replace_child(root, "neck", CubeListBuilder::create().tex_offs(0, 16).add_box_with([-3.0, -1.5, -3.0], [6.0, 3.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -4.0, -5.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.0, -1.1667, 3.6667], [2.0, 2.0, 11.0], CubeDeformation::new(0.0)).tex_offs(18, 13).add_box_with([-1.5, -1.1667, -0.3333], [3.0, 3.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -3.8333, -4.6667));
    let wings = mesh.add_or_replace_child(body, "wings", CubeListBuilder::create(), PartPose::offset(1.0, -1.1667, 5.1667));
    let wingsBack = mesh.add_or_replace_child(wings, "wingsBack", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let wingLeftBack = mesh.add_or_replace_child(wingsBack, "wingLeftBack", CubeListBuilder::create().tex_offs(12, 6).add_box_with([0.3378, 0.3686, -2.5], [9.0, 0.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, 0.0, -0.829));
    let wingRightBack = mesh.add_or_replace_child(wingsBack, "wingRightBack", CubeListBuilder::create().tex_offs(12, 6).set_mirror(true).add_box_with([-9.3378, 0.3686, -2.5], [9.0, 0.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(-2.0, 0.0, 0.0, 0.0, 0.0, 0.829));
    let wingsFront = mesh.add_or_replace_child(wings, "wingsFront", CubeListBuilder::create(), PartPose::offset(-2.0, 0.0, -4.0));
    let wingRightFront = mesh.add_or_replace_child(wingsFront, "wingRightFront", CubeListBuilder::create().tex_offs(12, 3).set_mirror(true).add_box_with([-10.9532, 0.2113, -1.5], [11.0, 0.0, 3.0], CubeDeformation::new(0.0)).set_mirror(false), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, 0.0, 0.4363));
    let wingLeftFront = mesh.add_or_replace_child(wingsFront, "wingLeftFront", CubeListBuilder::create().tex_offs(12, 3).add_box_with([-0.0468, 0.2113, -1.5], [11.0, 0.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(2.0, 0.0, 0.0, 0.0, 0.0, -0.4363));
    let legs = mesh.add_or_replace_child(body, "legs", CubeListBuilder::create(), PartPose::offset(0.0, 1.8333, 1.6667));
    let legsFront = mesh.add_or_replace_child(legs, "legsFront", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.5, 0.0, 0.0], [3.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, -1.0, 0.3927, 0.0, 0.0));
    let legsMiddle = mesh.add_or_replace_child(legs, "legsMiddle", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.5, 0.0, 0.0], [3.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.3927, 0.0, 0.0));
    let legsBack = mesh.add_or_replace_child(legs, "legsBack", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-1.5, 0.0, 0.0], [3.0, 2.0, 0.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 1.0, 0.3927, 0.0, 0.0));
    LayerDefinition::create(mesh, 32, 32).bake_root()
}
