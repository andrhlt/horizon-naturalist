fn clam_tick_motion_fixture(game: &mut Game<'_>) {
    use horizon_client::mobs::nav::naturalist_clam as clam;

    if game.mobs.iter().any(|mob| mob.kind.id == clam::KIND && clam::state(&mob.ai.nav).launch_delay == 1)
        && let Some(mut host) = game.lan.take()
    {
        let connections: Vec<_> = host.hub.guests.iter().map(|guest| guest.conn).collect();
        clam_motion_fixture::BEFORE.with_borrow_mut(|before| {
            before.clear();
            for id in connections {
                if let Some(Some(y)) =
                    game.as_guest(&mut host, id, |game| game.player.as_ref().map(|player| player.velocity.y))
                {
                    before.push((id, y));
                }
            }
        });
        game.lan = Some(host);
    }
    game.step_naturalist_clams();
}
