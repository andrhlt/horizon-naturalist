fn ostrich_ai(mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) -> std::ops::ControlFlow<()> {
    use super::naturalist_ostrich as ostrich;
    if mob.kind.id == ostrich::KIND {
        ostrich::tick(mob, world, events);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
