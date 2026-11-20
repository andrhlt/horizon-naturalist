fn clam_damage(mob: &Mob, source: &Damage) -> std::ops::ControlFlow<bool> {
    use nav::naturalist_clam as clam;
    if mob.kind.id == clam::KIND && !clam::state(&mob.ai.nav).open
        && !source.kind.bypasses_invulnerability()
        && !matches!(source.kind, DamageKind::Drown | DamageKind::InWall | DamageKind::Lava)
    {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
