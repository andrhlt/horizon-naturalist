fn ostrich_trample_actor(game: &mut Game<'_>, landing: &LandingVibration) {
    if landing.event == "minecraft:block_destroy"
        && game.pack.assets.registry.state(landing.state)
            .is_some_and(|row| row.name == horizon_client::mobs::naturalist_ostrich::EGG)
    {
        let pos = landing.position;
        naturalist_ostrich_game::egg_trampled(game, [pos.x, pos.y, pos.z]);
    }
}
