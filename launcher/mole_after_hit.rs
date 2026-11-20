fn mole_after_hit(hurt: bool, mob: &mut Mob, source: &Damage) -> bool {
    use self::goals::naturalist_mole_goals::{self as mole, MOLE};
    if hurt && mob.kind.id == MOLE && let Some(attacker) = source.attacker {
        mole::after_hit(mob, attacker);
    }
    hurt
}
