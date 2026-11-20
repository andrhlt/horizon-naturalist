fn aerial_parrot_glide(mob: &Mob) -> std::ops::ControlFlow<()> {
    if matches!(
        mob.kind.id.as_str(),
        "naturalist:butterfly"
            | "naturalist:dragonfly"
            | "naturalist:firefly"
            | "naturalist:vulture"
    ) {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
