fn ostrich_jump_start(mob: &horizon_client::mobs::Mob) -> std::ops::ControlFlow<Option<(&'static str, f32)>> {
    if mob.kind.id == "naturalist:ostrich" {
        return std::ops::ControlFlow::Break(None);
    }
    std::ops::ControlFlow::Continue(())
}
