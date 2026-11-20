fn deer_big_panic(
    goal: &mut Panic,
    mob: &mut Mob,
    context: &mut Cx<'_, '_>,
) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if !matches!(mob.kind.id.as_str(), "naturalist:deer" | "naturalist:giraffe") {
        return ControlFlow::Continue(());
    }
    if !should_panic(mob) {
        return ControlFlow::Break(false);
    }

    let world = context.w;
    let position = mob.position;
    let at = [position.x, position.y, position.z].map(|part| part.floor() as i32);
    if mob.fire_ticks > 0
        && world.collision_bounds(at).is_none()
        && let Some(water) = world.nearest_block(at, 10, 2, |[x, y, z], _| {
            world.fluid(x, y, z) == crate::blocks::Fluid::Water
        })
    {
        goal.pos = water.map(f64::from);
        return ControlFlow::Break(true);
    }
    let Some(position) = mob.default_random_pos(world, 15, 4) else {
        return ControlFlow::Break(false);
    };
    goal.pos = position;
    ControlFlow::Break(true)
}
