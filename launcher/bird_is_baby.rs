fn bird_is_baby(mob: &Mob) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    match mob.kind.id.as_str() {
        "naturalist:bird" | "naturalist:lizard_tail" => ControlFlow::Break(false),
        _ => ControlFlow::Continue(()),
    }
}
