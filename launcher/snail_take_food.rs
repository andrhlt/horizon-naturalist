fn snail_take_food(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == "naturalist:snail"
        && !mob.is_baby()
        && super::super::naturalist_snail::snail(mob).has_egg
    {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
