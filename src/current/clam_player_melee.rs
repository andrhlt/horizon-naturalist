fn clam_player_melee(game: &mut Game<'_>, index: usize) -> std::ops::ControlFlow<()> {
    let Some(mob) = game.mobs.get(index) else { return std::ops::ControlFlow::Continue(()) };
    if mob.kind.id == "naturalist:clam" && mob.extra.scratch[0] == 0 {
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
