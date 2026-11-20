fn boar_melee_gate(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if super::naturalist_bears::is_bear(&mob.kind.id) && !super::naturalist_bears::no_food(mob) {
        return std::ops::ControlFlow::Break(false);
    }
    if mob.kind.id == "naturalist:rhino" && (mob.ai.target == Some(super::Who::Player)
        || super::naturalist_rhino::stun_ticks(&mob.ai.nav) > 0)
    {
        return std::ops::ControlFlow::Break(false);
    }
    if mob.kind.id == "naturalist:boar"
        && (mob.is_baby() || mob.health <= mob.attrs.max_health * 0.5)
    {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
