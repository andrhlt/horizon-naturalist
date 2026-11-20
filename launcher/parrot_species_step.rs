fn parrot_species_step(mob: &Mob) -> std::ops::ControlFlow<()> {
    if matches!(mob.kind.id.as_str(), "naturalist:bird" | "naturalist:butterfly" | "naturalist:dragonfly" | "naturalist:firefly" | "naturalist:vulture") {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
