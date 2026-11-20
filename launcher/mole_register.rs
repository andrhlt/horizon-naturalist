fn mole_register(mob: &mut Mob) {
    use super::goals::naturalist_mole_goals::{self as mole, MOLE, TRAIL};
    if matches!(mob.kind.id.as_str(), MOLE | TRAIL) {
        mole::register(mob);
    }
}
