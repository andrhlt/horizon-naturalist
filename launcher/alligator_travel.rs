fn alligator_travel(
    mob: &mut Mob,
    world: &MobWorld<'_>,
    input: Vec3,
    events: &mut Vec<MobEvent>,
) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == "naturalist:whale" && !mob.no_ai && mob.fluids.in_water {
        mob.move_relative(mob.ai.speed, input);
        if input.y != 0.0 {
            let target = input.y * f64::from(mob.ai.speed) * 10.0;
            mob.velocity.y += (target - mob.velocity.y) * 0.25;
        }
        mob.do_move(world, events);
        mob.velocity = mob.velocity.scale(0.9);
        return std::ops::ControlFlow::Break(true);
    }
    if mob.kind.id != "naturalist:alligator" || mob.no_ai || !mob.fluids.in_water {
        return std::ops::ControlFlow::Continue(());
    }
    mob.move_relative(mob.ai.speed, input);
    mob.do_move(world, events);
    mob.velocity = mob.velocity.scale(0.9);
    if mob.ai.target.is_none() {
        mob.velocity.y -= 0.0025;
    }
    std::ops::ControlFlow::Break(true)
}
