fn deer_spawn_skip_donor(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    use std::ops::ControlFlow;
    if mob.kind.id == "naturalist:deer" && super::naturalist_deer_spawn::finished(mob) {
        ControlFlow::Break(())
    } else {
        ControlFlow::Continue(())
    }
}
