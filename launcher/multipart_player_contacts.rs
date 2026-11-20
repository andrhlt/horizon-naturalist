fn multipart_player_contacts(game: &mut Game<'_>) {
    use horizon_client::mobs::{PlayerView, nav::{naturalist_shark_state as shark, naturalist_whale_state as whale}};
    let mut parts = Vec::new();
    for mob in game.mobs.iter().filter(|mob| mob.alive() && game.creatures.ticked.contains(&mob.id)) {
        match mob.kind.id.as_str() {
            shark::KIND => {
                if let Some(state) = shark::motion(&mob.ai.nav)
                    && let Some(position) = state.contact_position
                {
                    parts.extend(shark::parts(position, f64::from(mob.height()), state.yaw, state.pitch));
                }
            }
            whale::KIND => {
                if let Some(boxes) = whale::contact_parts(mob) {
                    parts.extend(boxes);
                }
            }
            _ => {}
        }
    }
    if parts.is_empty() {
        return;
    }

    let impulses = |player: &PlayerView| {
        if !player.alive || player.spectator || player.sleeping || player.passenger {
            return Vec::new();
        }
        parts.iter()
            .filter(|part| part.intersects(player.bounds))
            .filter_map(|part| shark::part_impulse(player.position, part.center()))
            .collect::<Vec<_>>()
    };
    if let Some(view) = game.player_view()
        && let Some(player) = game.player.as_mut()
    {
        let before = player.velocity;
        for impulse in impulses(&view) {
            player.velocity = player.velocity.add_vector(impulse);
        }
        if player.velocity != before {
            player.health.sync_velocity = true;
        }
    }
    let guests = game.lan_guest_views();
    if let Some(mut lan) = game.lan.take() {
        for (conn, view) in guests {
            let pushes = impulses(&view);
            if !pushes.is_empty() {
                game.guest_hit_by(&mut lan, conn, |side| {
                    if let Some(player) = &mut side.player {
                        for impulse in pushes {
                            player.velocity = player.velocity.add_vector(impulse);
                        }
                    }
                });
            }
        }
        game.lan = Some(lan);
    }
}
