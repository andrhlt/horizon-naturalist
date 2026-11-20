fn rat_swim_sound(mob: &Mob) -> std::ops::ControlFlow<&'static str> {
    if mob.kind.id == "naturalist:rat" {
        return std::ops::ControlFlow::Break("naturalist:entity.rat.swim");
    }
    std::ops::ControlFlow::Continue(())
}
