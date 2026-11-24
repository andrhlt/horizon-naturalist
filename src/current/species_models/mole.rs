// Based on Naturalist 2.0.7 MoleModel bytecode and the pinned Java geometry.
use mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};

pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    let root = mesh.add_or_replace_child(root, "root", CubeListBuilder::create(), PartPose::offset(0.0, 24.0, 0.0));
    let body = mesh.add_or_replace_child(root, "body", CubeListBuilder::create()
        .tex_offs(0, 0).add_box_with([-4.0, -3.0, -4.5], [8.0, 6.0, 9.0], CubeDeformation::new(0.0)),
        PartPose::offset(0.0, -3.0, 1.5));
    mesh.add_or_replace_child(body, "tail", CubeListBuilder::create()
        .tex_offs(15, 19).add_box_with([-0.5, -0.5, 0.0], [1.0, 1.0, 6.0], CubeDeformation::new(0.0)),
        PartPose::offset(0.0, 2.5, 4.5));
    let skull = mesh.add_or_replace_child(body, "skull", CubeListBuilder::create()
        .tex_offs(0, 16).add_box_with([-3.0, -2.5, -3.6333], [6.0, 4.0, 4.0], CubeDeformation::new(0.0)),
        PartPose::offset(0.0, 1.5, -4.8667));
    mesh.add_or_replace_child(skull, "nose", CubeListBuilder::create()
        .tex_offs(26, 5).add_box_with([-2.5, -2.0, -2.1], [5.0, 3.0, 0.0], CubeDeformation::new(0.0))
        .tex_offs(26, 0).add_box_with([-1.5, -1.0, -2.0], [3.0, 2.0, 2.0], CubeDeformation::new(0.0)),
        PartPose::offset(0.0, 0.5, -3.6333));
    let legs = mesh.add_or_replace_child(root, "legs", CubeListBuilder::create(), PartPose::offset(4.0, -2.5, -2.5));
    mesh.add_or_replace_child(legs, "left_arm", CubeListBuilder::create()
        .tex_offs(24, 16).add_box_with([0.0, -2.5, -0.5], [6.0, 5.0, 1.0], CubeDeformation::new(0.0)),
        PartPose::offset(0.0, 0.0, 0.0));
    mesh.add_or_replace_child(legs, "right_arm", CubeListBuilder::create()
        .tex_offs(24, 16).set_mirror(true).add_box_with([-6.0, -2.5, -0.5], [6.0, 5.0, 1.0], CubeDeformation::new(0.0)).set_mirror(false),
        PartPose::offset(-8.0, 0.0, 0.0));
    mesh.add_or_replace_child(legs, "left_leg", CubeListBuilder::create()
        .tex_offs(0, 25).add_box_with([0.0, -1.5, -0.5], [4.0, 3.0, 2.0], CubeDeformation::new(0.0)),
        PartPose::offset(0.0, 1.0, 8.0));
    mesh.add_or_replace_child(legs, "right_leg", CubeListBuilder::create()
        .tex_offs(0, 25).set_mirror(true).add_box_with([-4.0, -1.5, -0.5], [4.0, 3.0, 2.0], CubeDeformation::new(0.0)).set_mirror(false),
        PartPose::offset(-8.0, 1.0, 8.0));
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
