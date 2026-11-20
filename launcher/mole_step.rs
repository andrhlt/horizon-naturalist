fn mole_step(mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) {
    use super::goals::naturalist_mole_goals::{self as mole, MOLE};
    if mob.kind.id == MOLE && mob.alive() {
        mole::step(mob, world, events);
    }
}
