fn duck_egg_hatch(game: &mut Game<'_>, projectile: &Projectile, hit: Hit) {
    let hit_position = match hit {
        Hit::Block { at, .. } | Hit::Entity { at, .. } => at,
        Hit::None => return,
    };
    if !matches!(&projectile.kind, ProjectileKind::Thrown(kind) if kind.id == "naturalist:duck_egg") {
        return;
    }
    let (mut random, _) = horizon_client::projectiles::new_own_random();
    if random.next_int_bound(8).unwrap_or(1) != 0 {
        return;
    }
    let count = if random.next_int_bound(32).unwrap_or(1) == 0 { 4 } else { 1 };
    let kind = "naturalist:duck";
    let pos = [hit_position.x, hit_position.y, hit_position.z];
    for _ in 0..count {
        if let Some(id) = game.spawn_mob_raw(kind, pos, false)
            && let Some(duck) = game.mobs.iter_mut().find(|mob| mob.id == id)
        {
            horizon_client::mobs::finalize::set_age(duck, -24000);
            duck.yaw = projectile.yaw;
            duck.head_yaw = projectile.yaw;
            duck.body_yaw = projectile.yaw;
        }
    }
}
