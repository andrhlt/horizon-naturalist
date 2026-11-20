fn dragonfly_client_water_fx(mob: &Mob) -> std::ops::ControlFlow<()> {
    if mob.kind.id == "naturalist:dragonfly" {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
