fn zebra_equine_finalize(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    use super::naturalist_zebra as zebra;
    if mob.kind.id == "naturalist:ostrich" {
        return std::ops::ControlFlow::Break(());
    }
    if mob.kind.id == zebra::KIND {
        let stats = zebra::roll(&mut mob.random);
        zebra::apply_roll(mob, stats);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
