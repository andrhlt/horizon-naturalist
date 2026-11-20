fn whale_air(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    if mob.kind.id == super::super::nav::naturalist_whale_state::KIND
        && mob.alive()
        && mob.fluids.in_water
    {
        mob.air = mob.max_air_supply();
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
