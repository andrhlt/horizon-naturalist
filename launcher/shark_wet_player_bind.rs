fn shark_wet_player_bind(mob: &Mob) -> std::ops::ControlFlow<Option<u32>> {
    use horizon_client::mobs::nav::naturalist_shark_state as shark;

    if mob.kind.id == shark::KIND
        && let Some(choice) = shark::motion(&mob.ai.nav).and_then(|motion| motion.wet_player)
    {
        return std::ops::ControlFlow::Break(choice);
    }
    std::ops::ControlFlow::Continue(())
}
