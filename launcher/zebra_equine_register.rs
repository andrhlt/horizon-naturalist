fn zebra_equine_register(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    use super::{naturalist_ostrich as ostrich, naturalist_zebra as zebra};
    if mob.kind.id == ostrich::KIND {
        ostrich::register(mob);
        return std::ops::ControlFlow::Break(());
    }
    if mob.kind.id == zebra::KIND {
        zebra::register(mob);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
