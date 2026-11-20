fn duck_bucket_persistence(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if mob.kind.id == "naturalist:duck" && mob.extra.from_bucket {
        return std::ops::ControlFlow::Break(true);
    }
    std::ops::ControlFlow::Continue(())
}
