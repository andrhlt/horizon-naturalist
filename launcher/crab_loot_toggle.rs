fn crab_loot_toggle(mob: &Mob) -> std::ops::ControlFlow<bool> {
    use nav::naturalist_crab as crab;
    if mob.kind.id == crab::KIND {
        return std::ops::ControlFlow::Break(!crab::state(&mob.ai.nav).no_pickup);
    }
    std::ops::ControlFlow::Continue(())
}
