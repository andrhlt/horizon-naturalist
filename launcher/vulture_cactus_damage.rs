fn vulture_cactus_damage(mob: &Mob, source: &Damage) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == "naturalist:hedgehog"
        && (matches!(source.kind, DamageKind::Cactus | DamageKind::SweetBerryBush)
            || (source.is_fire() && naturalist_hedgehog::state(mob).rolling))
    {
        return std::ops::ControlFlow::Break(true);
    }
    if mob.kind.id == "naturalist:vulture" && source.kind == DamageKind::Cactus {
        return std::ops::ControlFlow::Break(true);
    }
    std::ops::ControlFlow::Continue(())
}
