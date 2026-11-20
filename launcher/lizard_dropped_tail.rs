fn lizard_dropped_tail(game: &mut Game<'_>) {
    use horizon_client::mobs::{Who, naturalist_lizard};
    use horizon_core::geometry::{Aabb, Vec3};
    let tails: Vec<_> = game.mobs.iter_mut().filter_map(|mob| {
        (mob.kind.id == "naturalist:lizard" && naturalist_lizard::take_tail_drop(&mut mob.ai.nav))
            .then(|| (mob.id, mob.position, mob.bounds(), mob.silent))
    }).collect();
    for (lizard, position, bounds, silent) in tails {
        let at = [position.x, position.y, position.z];
        let variant = horizon_client_core::nbt_value::text(
            game.persist.raw.get(&lizard).and_then(|tag| tag.get("Variant")),
        ).unwrap_or_else(|| "naturalist:green".to_owned());
        if !silent {
            game.audio.play("minecraft:entity.slime.squish", Some(at), 1.0, 1.0);
        }
        let Some(tail) = game.spawn_mob_raw("naturalist:lizard_tail", at, false) else { continue };
        super::naturalist_deer_variants::put_variant(game, tail, &variant);
        let offset = Vec3::new(8.0, 8.0, 8.0);
        let nearby = Aabb::new(bounds.min_position().subtract(offset), bounds.max_position().add_vector(offset));
        for mob in game.mobs.iter_mut().filter(|mob| mob.ai.target == Some(Who::Mob(lizard))) {
            if nearby.intersects(mob.bounds()) {
                mob.ai.target = Some(Who::Mob(tail));
            }
        }
    }
}
