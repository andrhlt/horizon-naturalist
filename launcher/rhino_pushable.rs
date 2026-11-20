fn rhino_pushable(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == "naturalist:rhino" && super::naturalist_rhino::preparing(&mob.ai.nav) {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
