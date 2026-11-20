fn mole_damage(mob: &Mob, source: &Damage) -> std::ops::ControlFlow<bool> {
    use self::goals::naturalist_mole_goals::{self as mole, MOLE, Phase, TRAIL};
    if mob.kind.id == TRAIL || mob.kind.id == MOLE
        && !matches!(mole::phase(mob), Phase::Unrolled | Phase::Peeking)
        && !source.kind.bypasses_invulnerability()
    {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
