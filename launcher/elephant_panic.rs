fn elephant_panic(mob: &Mob) -> std::ops::ControlFlow<bool> {
    use super::nav::naturalist_elephants;
    if naturalist_elephants::includes(&mob.kind.id) && !mob.is_baby() && mob.fire_ticks <= 0 {
        std::ops::ControlFlow::Break(false)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
