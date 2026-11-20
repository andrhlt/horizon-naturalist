fn lion_pride_update(game: &mut Game<'_>) {
    use horizon_client::mobs::goals::naturalist_lion_goals as lion;
    use horizon_core::geometry::Vec3;

    let adults = game
        .mobs
        .iter()
        .enumerate()
        .filter(|(_, mob)| {
            mob.kind.id == lion::KIND && mob.alive() && !mob.is_baby() && !mob.removed
        })
        .filter_map(|(index, mob)| {
            game.persist
                .uuids
                .get(&mob.id)
                .map(|&uuid| (index, lion::ranking(uuid)))
        })
        .collect::<Vec<_>>();
    for &(index, rank) in &adults {
        let mob = &game.mobs[index];
        let neighborhood = mob.bounds().inflate(Vec3::new(24.0, 24.0, 24.0));
        let candidate = adults
            .iter()
            .filter(|&&(other_index, other_rank)| {
                let other = &game.mobs[other_index];
                other_index != index
                    && !other.extra.flag
                    && !other.is_invisible()
                    && (mob.extra.flag || other_rank < rank)
                    && neighborhood.intersects(other.bounds())
            })
            .min_by_key(|&&(_, rank)| rank)
            .map_or((None, [0.0; 3]), |&(other, _)| {
                let leader = &game.mobs[other];
                (Some(leader.id), leader.ai.look.wanted)
            });
        lion::put_pride(&mut game.mobs[index].ai.nav, candidate.0, candidate.1);
    }
}
