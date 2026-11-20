fn giant_isopod_persistence(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == "naturalist:giant_isopod" && mob.extra.from_bucket {
        return std::ops::ControlFlow::Break(true);
    }
    std::ops::ControlFlow::Continue(())
}
