fn lion_finalize_later(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    use super::goals::naturalist_lion_goals as lion;
    if mob.kind.id == lion::KIND {
        lion::mark_ordinary_spawn(mob);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
