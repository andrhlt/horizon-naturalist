fn mob_xp_overrides(mob: &mut Mob) -> std::ops::ControlFlow<i32> {
    use std::ops::ControlFlow;

    match mob.kind.id.as_str() {
        "naturalist:duck" => ControlFlow::Break(10),
        "naturalist:vulture" | "naturalist:dragonfly" | "naturalist:lizard_tail" => {
            ControlFlow::Break(0)
        }
        _ => ControlFlow::Continue(()),
    }
}
