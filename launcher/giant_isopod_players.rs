fn giant_isopod_players(game: &mut Game<'_>) {
    use horizon_client::mobs::nav::naturalist_giant_isopod as isopod;

    if !game.mobs.iter().any(|mob| mob.kind.id == isopod::KIND) {
        return;
    }
    let guests = game.lan_guest_views();
    let world = game.mob_world_base(&[]);
    let nearby: Vec<_> = game.mobs.iter().enumerate()
        .filter(|(_, mob)| mob.kind.id == isopod::KIND)
        .map(|(index, mob)| {
            let nearby = world.player.iter().chain(guests.iter().map(|(_, player)| player))
                .any(|player| isopod::player_visible(mob, player, &world));
            (index, nearby)
        }).collect();
    for (index, nearby) in nearby {
        isopod::state_mut(&mut game.mobs[index].ai.nav).nearby = nearby;
    }
}
