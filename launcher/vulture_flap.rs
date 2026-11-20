fn vulture_flap(mob: &Mob) -> std::ops::ControlFlow<bool> {
    match mob.kind.id.as_str() {
        "naturalist:butterfly" => {
            std::ops::ControlFlow::Break(!mob.on_ground && mob.age.is_multiple_of(2))
        }
        "naturalist:firefly" => std::ops::ControlFlow::Break(mob.age.is_multiple_of(2)),
        "naturalist:dragonfly" | "naturalist:vulture" => std::ops::ControlFlow::Break(false),
        _ => std::ops::ControlFlow::Continue(()),
    }
}
