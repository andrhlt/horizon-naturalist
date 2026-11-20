fn boar_panic(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if matches!(mob.kind.id.as_str(), "naturalist:boar" | "naturalist:komodo_dragon" | "naturalist:rhino" | "naturalist:bear" | "naturalist:black_bear") {
        return std::ops::ControlFlow::Break(mob.fire_ticks > 0
            || (mob.is_baby() && mob.ai.last_hurt_by.is_some()));
    }
    std::ops::ControlFlow::Continue(())
}
