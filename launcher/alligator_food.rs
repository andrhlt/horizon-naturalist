fn alligator_food(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if matches!(mob.kind.id.as_str(), "naturalist:alligator" | "naturalist:tortoise") && mob.extra.flag && !mob.is_baby() {
        std::ops::ControlFlow::Break(false)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
