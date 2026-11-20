fn ostrich_ride(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    use super::naturalist_ostrich as ostrich;
    if mob.kind.id == ostrich::KIND {
        ostrich::ride(mob);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
