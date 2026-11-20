fn ostrich_attach(mob: &Mob) -> std::ops::ControlFlow<[f64; 3]> {
    use super::naturalist_ostrich as ostrich;
    if mob.kind.id == ostrich::KIND {
        return std::ops::ControlFlow::Break(ostrich::attachment(mob));
    }
    std::ops::ControlFlow::Continue(())
}
