// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(partdefinition, "root", CubeListBuilder::create(), PartPose::offset(0.0, 21.0, 10.0));
    let top = mesh.add_or_replace_child(root, "top", CubeListBuilder::create().tex_offs(0, 26).add_box_with([-12.5, -5.0, -20.0], [25.0, 5.0, 20.0], CubeDeformation::new(0.0)).tex_offs(8, 53).add_box_with([-9.5, -5.0, -25.0], [19.0, 5.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let bottom = mesh.add_or_replace_child(root, "bottom", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-12.5, 0.0, -20.0], [25.0, 3.0, 20.0], CubeDeformation::new(0.0)).tex_offs(72, 10).add_box_with([-9.5, 0.0, -25.0], [19.0, 3.0, 5.0], CubeDeformation::new(0.0)).tex_offs(76, 29).add_box_with([-6.5, -2.0, -15.0], [13.0, 2.0, 12.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let rightArm = mesh.add_or_replace_child(bottom, "rightArm", CubeListBuilder::create(), PartPose::offset(0.0, 0.0, 0.0));
    let rightItem = mesh.add_or_replace_child(rightArm, "rightItem", CubeListBuilder::create(), PartPose::offset(0.0, -2.0, -10.0));
    let hinge = mesh.add_or_replace_child(root, "hinge", CubeListBuilder::create().tex_offs(59, 53).add_box_with([-6.5, -2.0, -0.5], [13.0, 4.0, 5.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.5));
    LayerDefinition::create(mesh, 128, 128).bake_root()
}
