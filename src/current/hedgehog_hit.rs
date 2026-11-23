fn hedgehog_hit(game: &mut Game<'_>, projectile: &Projectile, hit: Hit) {
    crate::game::naturalist_capture_net::hedgehog_hit(game, projectile, hit);
}
