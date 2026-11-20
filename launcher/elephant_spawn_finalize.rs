fn elephant_spawn_finalize(mob: &Mob) -> std::ops::ControlFlow<()> {
    if super::nav::naturalist_elephants::includes(&mob.kind.id) {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
