fn piranha_hit(hurt: bool, mob: &mut Mob, source: Damage) -> bool {
    use crate::mobs::nav::naturalist_piranha_state as piranha;
    if hurt && matches!(mob.kind.id.as_str(), "naturalist:piranha" | "naturalist:ray" | "naturalist:anglerfish")
        && let Some(attacker) = source.attacker.filter(|who| *who != Who::Mob(mob.id))
    {
        // Enemy-player acceptance needs the authoritative world's difficulty.
        piranha::hold_retaliation(&mut mob.ai.nav, attacker);
        if matches!(attacker, Who::Mob(_)) {
            mob.ai.target = Some(attacker);
        }
    }
    hurt
}
