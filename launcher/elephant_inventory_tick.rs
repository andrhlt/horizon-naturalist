fn elephant_inventory_tick(game: &mut Game<'_>) {
    if game.remote.is_some() {
        return;
    }
    let Some((id, _)) = game.naturalist_elephant_inventory() else { return };
    let actor = game.acting_player_uuid();
    let valid = game.mobs.iter().find(|mob| mob.id == id).is_some_and(|mob| {
        mob.alive() && mob.extra.tame && mob.extra.owner == Some(actor)
            && game.player.as_ref().is_some_and(|player| {
                mob.distance_sqr(player.position) < 64.0
            })
    });
    if valid {
        game.sync_local_naturalist_elephant_inventory();
    } else {
        game.close_container();
        game.screen = Screen::Playing;
        game.capture(true);
    }
}
