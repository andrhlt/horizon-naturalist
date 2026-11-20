fn mole_push(mob: &Mob) -> std::ops::ControlFlow<bool> {
    use super::goals::naturalist_mole_goals::{self as mole, MOLE, Phase, TRAIL};
    if mob.kind.id == TRAIL || mob.kind.id == MOLE && mole::phase(mob) != Phase::Unrolled {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
