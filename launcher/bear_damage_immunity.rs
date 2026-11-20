fn bear_damage_immunity(mob: &Mob, source: &Damage) -> std::ops::ControlFlow<bool> {
    if naturalist_bears::is_bear(&mob.kind.id) && source.kind == DamageKind::SweetBerryBush {
        return std::ops::ControlFlow::Break(true);
    }
    if mob.kind.id == "naturalist:whale" && source.attacker_species == Some(Species::Pufferfish) {
        return std::ops::ControlFlow::Break(true);
    }
    std::ops::ControlFlow::Continue(())
}
