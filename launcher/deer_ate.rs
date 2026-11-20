fn deer_ate(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    use std::ops::ControlFlow;
    if mob.kind.id != "naturalist:deer" {
        return ControlFlow::Continue(());
    }
    if can_age_up(mob) {
        age_up(mob, 60, false);
    }
    ControlFlow::Break(())
}
