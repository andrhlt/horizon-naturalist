fn alligator_walk_value(
    mob: &Mob,
    world: &MobWorld<'_>,
    pos: [i32; 3],
) -> std::ops::ControlFlow<f32> {
    if mob.kind.id == "naturalist:alligator"
        && mob.fluids.in_water
        && world.fluid(pos[0], pos[1], pos[2]) == crate::blocks::Fluid::Water
    {
        std::ops::ControlFlow::Break(10.0)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
