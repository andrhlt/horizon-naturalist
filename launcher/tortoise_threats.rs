fn tortoise_threats(game: &mut Game<'_>) {
    use horizon_client::mobs::goals::naturalist_tortoise_goals as tortoise;
    if game.remote.is_some() || !game.mobs.iter().any(|mob| mob.kind.id == tortoise::KIND && mob.alive()) {
        return;
    }
    let guests = game.lan_guest_views();
    let host = game.player_view();
    let world = game.mob_world_base(&[]);
    let updates: Vec<_> = game.mobs.iter().enumerate().filter(|(_, mob)| mob.kind.id == tortoise::KIND && mob.alive())
        .map(|(index, mob)| (index, host.iter().chain(guests.iter().map(|(_, view)| view))
            .any(|player| tortoise::can_hide(mob, &world, player))))
        .collect();
    for (index, hidden) in updates {
        tortoise::set_hidden(&mut game.mobs[index], hidden);
    }
}
