fn crab_armed(mob: &Mob) -> std::ops::ControlFlow<bool> {
    use super::nav::naturalist_crab as crab;
    if mob.kind.id == crab::KIND && !crab::holding_weapon(mob) {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
