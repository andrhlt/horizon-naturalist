fn ant_hill_releases(game: &mut Game<'_>) {
    use crate::game::naturalist_ant_hill::RELEASE_EVENTS;
    let mut exits = Vec::new();
    game.level_events.retain(|event| {
        if let LevelEvent::BlockEvent { pos, kind, .. } = event
            && let Some(index) = RELEASE_EVENTS
                .iter()
                .position(|candidate| candidate == kind)
        {
            exits.push(([pos.x, pos.y, pos.z], index));
            false
        } else {
            true
        }
    });
    for (position, index) in exits {
        crate::game::naturalist_ant_hill::release_one(game, position, index);
    }
}
