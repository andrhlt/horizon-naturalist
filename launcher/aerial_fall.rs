fn aerial_fall(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if matches!(mob.kind.id.as_str(), "naturalist:ostrich" | "naturalist:hedgehog" | "naturalist:dragonfly" | "naturalist:firefly" | "naturalist:butterfly" | "naturalist:caterpillar" | "naturalist:vulture" | "naturalist:snail") {
        std::ops::ControlFlow::Break(false)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
