fn antivenom_flush_splashes(game: &mut Game<'_>) {
    if game.lan.is_none() {
        return;
    }
    let Ok(Some(pending)) = game.world.attachments.remove(&super::naturalist_antivenom::PENDING_SPLASHES) else {
        return;
    };
    for (at, age, handled) in pending {
        game.for_each_player(|game| {
            if game.lan_acting != Some(handled) {
                super::potions::naturalist_antivenom_splash::clear_player_at(game, at, age);
            }
        });
    }
}
