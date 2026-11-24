// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let mound = mesh.add_or_replace_child(partdefinition, "mound", CubeListBuilder::create().tex_offs(4, 28).add_box_with([-5.0, -3.5, -5.0], [10.0, 3.5, 10.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 24.0, 0.0));
    let extra1 = mesh.add_or_replace_child(mound, "extra1", CubeListBuilder::create(), PartPose::offset(-3.5, -1.9, -4.0));
    let extra1_r1 = mesh.add_or_replace_child(extra1, "extra1_r1", CubeListBuilder::create().tex_offs(35, 29).add_box_with([-2.5, -2.5, -2.0], [5.0, 4.5, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, -1.1781, 0.0));
    let extra1_r2 = mesh.add_or_replace_child(extra1, "extra1_r2", CubeListBuilder::create().tex_offs(45, 38).add_box_with([-1.5, -1.5, -1.5], [3.0, 2.5, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(5.0, 1.0, -1.5, 0.0, 0.7854, 0.0));
    let extra2 = mesh.add_or_replace_child(mound, "extra2", CubeListBuilder::create(), PartPose::offset(-5.0, -0.9, 1.5));
    let extra2_r1 = mesh.add_or_replace_child(extra2, "extra2_r1", CubeListBuilder::create().tex_offs(45, 38).add_box_with([-1.5, -1.5, -1.5], [3.0, 2.5, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, 0.7854, 0.0));
    let extra3 = mesh.add_or_replace_child(mound, "extra3", CubeListBuilder::create().tex_offs(45, 38).add_box_with([4.0, -2.4, -1.0], [3.0, 2.5, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let extra3_r1 = mesh.add_or_replace_child(extra3, "extra3_r1", CubeListBuilder::create().tex_offs(35, 29).add_box_with([-2.5, -2.5, -2.0], [5.0, 4.5, 4.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(3.5, -1.9, 4.0, 0.6155, 0.5236, 0.9553));
    let extra3_r2 = mesh.add_or_replace_child(extra3, "extra3_r2", CubeListBuilder::create().tex_offs(45, 38).add_box_with([-1.5, -1.5, -1.5], [3.0, 2.5, 3.0], CubeDeformation::new(0.0)), PartPose::offset_and_rotation(0.5, -0.9, 5.5, 0.0, -0.3927, 0.0));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
