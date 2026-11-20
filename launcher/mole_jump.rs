fn mole_jump(mob: &Mob) -> std::ops::ControlFlow<()> {
    use super::goals::naturalist_mole_goals::{self as mole, MOLE, Phase};
    if mob.kind.id == MOLE && mole::phase(mob) != Phase::Unrolled {
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
