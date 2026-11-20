fn elephant_anger_lan_view(
    mob: &Mob,
    host: Option<&PlayerView>,
    guests: &GuestViews,
) -> std::ops::ControlFlow<Option<u32>> {
    use horizon_client::mobs::nav::naturalist_elephants as elephant;

    if elephant::includes(&mob.kind.id)
        && let Some(uuid) = elephant::anger_player_to_view(mob)
    {
        // Keep the actual player visible when they become a spectator so
        // NeutralMob can clear their anger rather than transfer it.
        if host.is_some_and(|player| player.uuid == uuid) {
            return std::ops::ControlFlow::Break(None);
        }
        if let Some((conn, _)) = guests.iter().find(|(_, player)| player.uuid == uuid) {
            return std::ops::ControlFlow::Break(Some(*conn));
        }
    }
    std::ops::ControlFlow::Continue(())
}
