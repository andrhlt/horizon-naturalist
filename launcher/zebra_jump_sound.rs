fn zebra_jump_sound(mob: &Mob) -> std::ops::ControlFlow<&'static str> {
    if mob.kind.id == super::naturalist_zebra::KIND {
        return std::ops::ControlFlow::Break("naturalist:entity.zebra.jump");
    }
    if mob.kind.id == super::naturalist_ostrich::KIND {
        return std::ops::ControlFlow::Break("intentionally_empty");
    }
    std::ops::ControlFlow::Continue(())
}
