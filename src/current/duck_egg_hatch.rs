fn duck_egg_hatch(game: &mut Game<'_>, projectile: &Projectile, hit: Hit) {
    use horizon_core::random::RandomSource;
    let at = match hit {
        Hit::Block { at, .. } | Hit::Entity { at, .. } => at,
        Hit::None => return,
    };
    if !matches!(&projectile.kind, ProjectileKind::Thrown(kind) if kind.id == "naturalist:duck_egg")
    {
        return;
    }
    // The projectile is discarded after this hit, so use a clone of its own
    // RNG stream to make precisely the same draws without touching the level RNG.
    let mut random = projectile.random.0.clone();
    if random.next_int_bound(8).unwrap_or(1) != 0 {
        return;
    }
    let count = if random.next_int_bound(32).unwrap_or(1) == 0 {
        4
    } else {
        1
    };
    for _ in 0..count {
        if let Some(id) = game.spawn_mob_raw("naturalist:duck", [at.x, at.y, at.z], false)
            && let Some(duck) = game.mobs.iter_mut().find(|mob| mob.id == id)
        {
            horizon_client::mobs::finalize::set_age(duck, -24000, None);
            duck.yaw = projectile.yaw;
            duck.head_yaw = projectile.yaw;
            duck.body_yaw = projectile.yaw;
        }
    }
}
