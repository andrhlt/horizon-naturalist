// Based on Naturalist 2.0.7 Java model source (MIT).
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let main = mesh.add_or_replace_child(partdefinition, "main", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let neck = mesh.add_or_replace_child(main, "neck", CubeListBuilder::create().tex_offs(0, 15).add_box_with([-1.5, -5.75, -1.5], [3.0, 7.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.25, -3.5));
    let skull = mesh.add_or_replace_child(neck, "skull", CubeListBuilder::create().tex_offs(12, 9).add_box_with([-2.0, -1.0, -1.0], [4.0, 1.0, 2.0], CubeDeformation::new(0.0)).tex_offs(0, 9).add_box_with([-1.5, -1.0, -5.0], [3.0, 1.0, 4.0], CubeDeformation::new(0.0)).tex_offs(31, 21).add_box_with([-3.0, -5.0, -5.0], [6.0, 2.0, 6.0], CubeDeformation::new(0.0)).tex_offs(28, 57).add_box_with([-3.0, -1.0, -1.0], [6.0, 1.0, 2.0], CubeDeformation::new(0.0)).tex_offs(0, 47).add_box_with([-2.5, -1.0, -5.0], [5.0, 1.0, 4.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -5.75, 0.5));
    let jaw = mesh.add_or_replace_child(skull, "jaw", CubeListBuilder::create().tex_offs(21, 9).add_box_with([-2.0, 0.0, -4.0], [4.0, 1.0, 4.0], CubeDeformation::new(0.01)).tex_offs(7, 55).add_box_with([-3.0, 0.0, -4.0], [6.0, 1.0, 4.0], CubeDeformation::new(0.01)), PartPose::offset(0.0, -1.0, -1.0));
    let tongue = mesh.add_or_replace_child(jaw, "tongue", CubeListBuilder::create().tex_offs(21, 2).add_box_with([-0.5, 0.0, -3.0], [1.0, 0.0, 3.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -0.1, 0.0));
    let awake = mesh.add_or_replace_child(skull, "awake", CubeListBuilder::create().tex_offs(21, 0).add_box_with([-2.0, -10.0, -8.0], [4.0, 2.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 7.0, 3.0));
    let awakeWide = mesh.add_or_replace_child(awake, "awakeWide", CubeListBuilder::create().tex_offs(12, 46).add_box_with([-3.0, -10.0, -8.0], [6.0, 2.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let asleep = mesh.add_or_replace_child(skull, "asleep", CubeListBuilder::create().tex_offs(0, 0).add_box_with([-2.0, -10.0, -8.0], [4.0, 2.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 7.0, 3.0));
    let asleepWide = mesh.add_or_replace_child(asleep, "asleepWide", CubeListBuilder::create().tex_offs(37, 46).add_box_with([-3.0, -10.0, -8.0], [6.0, 2.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.0, 0.0));
    let tail = mesh.add_or_replace_child(main, "tail", CubeListBuilder::create().tex_offs(13, 16).add_box_with([-2.0, -1.5, 0.0], [4.0, 3.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, -1.5, -2.0));
    let tail2 = mesh.add_or_replace_child(tail, "tail2", CubeListBuilder::create().tex_offs(32, 11).add_box_with([-2.0, -1.5, 0.0], [4.0, 3.0, 6.0], CubeDeformation::new(0.01)), PartPose::offset(0.0, 0.0, 6.0));
    let tail3 = mesh.add_or_replace_child(tail2, "tail3", CubeListBuilder::create().tex_offs(48, 7).add_box_with([-1.0, -1.0, 0.0], [2.0, 2.0, 6.0], CubeDeformation::new(0.0)), PartPose::offset(0.0, 0.5, 6.0));
    let tail4 = mesh.add_or_replace_child(tail3, "tail4", CubeListBuilder::create().tex_offs(50, 17).add_box_with([-1.0, -1.0, 0.0], [2.0, 2.0, 4.0], CubeDeformation::new(0.01)), PartPose::offset(0.0, 0.0, 6.0));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
