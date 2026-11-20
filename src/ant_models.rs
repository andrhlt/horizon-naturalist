// Based on Naturalist 2.0.7 Java model source (MIT).
use horizon_client::mobs::model_layers::{
    CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose,
};
pub(super) fn adult() -> ModelPart {
    let mut mesh = MeshDefinition::new();
    let partdefinition = mesh.root();
    let root = mesh.add_or_replace_child(
        partdefinition,
        "root",
        CubeListBuilder::create(),
        PartPose::offset(0.0, 24.0, 0.0),
    );
    let body = mesh.add_or_replace_child(
        root,
        "body",
        CubeListBuilder::create()
            .tex_offs(0, 21)
            .add_box_with(
                [-1.5, -3.5, -3.0],
                [3.0, 4.0, 6.0],
                CubeDeformation::new(0.0),
            )
            .tex_offs(0, 0)
            .add_box_with(
                [-2.5, -5.5, 0.0],
                [5.0, 5.0, 6.0],
                CubeDeformation::new(0.01),
            ),
        PartPose::offset(0.0, -2.5, 1.0),
    );
    let skull = mesh.add_or_replace_child(
        body,
        "skull",
        CubeListBuilder::create().tex_offs(0, 11).add_box_with(
            [-2.5, -2.5, -6.0],
            [5.0, 5.0, 5.0],
            CubeDeformation::new(0.0),
        ),
        PartPose::offset(0.0, -3.0, -2.0),
    );
    let antennae = mesh.add_or_replace_child(
        skull,
        "antennae",
        CubeListBuilder::create().tex_offs(17, 19).add_box_with(
            [-2.5, -5.0, -3.0],
            [5.0, 5.0, 3.0],
            CubeDeformation::new(0.0),
        ),
        PartPose::offset_and_rotation(0.0, -2.5, -5.0, -0.3927, 0.0, 0.0),
    );
    let legs = mesh.add_or_replace_child(
        root,
        "legs",
        CubeListBuilder::create(),
        PartPose::offset(0.0, -0.5, -0.5),
    );
    let left_legs = mesh.add_or_replace_child(
        legs,
        "left_legs",
        CubeListBuilder::create(),
        PartPose::offset(0.0, 0.0, 1.0),
    );
    let left_middle = mesh.add_or_replace_child(
        left_legs,
        "left_middle",
        CubeListBuilder::create().tex_offs(17, 4).add_box_with(
            [-0.5, 0.0, -0.5],
            [4.0, 0.0, 1.0],
            CubeDeformation::new(0.0),
        ),
        PartPose::offset_and_rotation(1.5, -2.0, 0.5, 0.0, 0.0, 0.7854),
    );
    let left_front = mesh.add_or_replace_child(
        left_legs,
        "left_front",
        CubeListBuilder::create().tex_offs(17, 4).add_box_with(
            [-0.5, 0.0, -0.5],
            [4.0, 0.0, 1.0],
            CubeDeformation::new(0.0),
        ),
        PartPose::offset_and_rotation(1.5, -1.75, -2.25, 0.0, 0.6109, 0.7854),
    );
    let left_back = mesh.add_or_replace_child(
        left_legs,
        "left_back",
        CubeListBuilder::create().tex_offs(17, 4).add_box_with(
            [-0.5, 0.0, -0.5],
            [4.0, 0.0, 1.0],
            CubeDeformation::new(0.0),
        ),
        PartPose::offset_and_rotation(1.5, -1.75, 3.25, 0.0, -0.6109, 0.7854),
    );
    let right_legs = mesh.add_or_replace_child(
        legs,
        "right_legs",
        CubeListBuilder::create(),
        PartPose::offset(0.0, 0.0, 1.0),
    );
    let right_middle = mesh.add_or_replace_child(
        right_legs,
        "right_middle",
        CubeListBuilder::create()
            .tex_offs(17, 4)
            .set_mirror(true)
            .add_box_with(
                [-3.5, 0.0, -0.5],
                [4.0, 0.0, 1.0],
                CubeDeformation::new(0.0),
            )
            .set_mirror(false),
        PartPose::offset_and_rotation(-1.5, -2.0, 0.5, 0.0, 0.0, -0.7854),
    );
    let right_front = mesh.add_or_replace_child(
        right_legs,
        "right_front",
        CubeListBuilder::create()
            .tex_offs(17, 4)
            .set_mirror(true)
            .add_box_with(
                [-3.5, 0.0, -0.5],
                [4.0, 0.0, 1.0],
                CubeDeformation::new(0.0),
            )
            .set_mirror(false),
        PartPose::offset_and_rotation(-1.5, -1.75, -2.25, 0.0, -0.6109, -0.7854),
    );
    let right_back = mesh.add_or_replace_child(
        right_legs,
        "right_back",
        CubeListBuilder::create()
            .tex_offs(17, 4)
            .set_mirror(true)
            .add_box_with(
                [-3.5, 0.0, -0.5],
                [4.0, 0.0, 1.0],
                CubeDeformation::new(0.0),
            )
            .set_mirror(false),
        PartPose::offset_and_rotation(-1.5, -1.75, 3.25, 0.0, 0.6109, -0.7854),
    );
    LayerDefinition::create(mesh, 64, 64).bake_root()
}
