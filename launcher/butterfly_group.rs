fn butterfly_group(
    game: &Game<'_>,
    request: &SpawnRequest<'_>,
    pending: &mut Vec<Mob>,
) -> std::ops::ControlFlow<Option<Spawned>> {
    use horizon_client::mobs::{raider, spawning};
    use horizon_core::random::RandomSource;
    use std::ops::ControlFlow;

    if request.entity != "naturalist:butterfly" {
        return ControlFlow::Continue(());
    }
    let Some(kind) = game.res.mob_types().get(request.entity).cloned() else {
        return ControlFlow::Break(None);
    };
    let [x, y, z] = request.position;
    let mut butterfly = Mob::new(0, kind, Vec3::new(x, y, z), request.yaw);
    let facts = game.biome_facts(x.floor() as i32, y.floor() as i32, z.floor() as i32);
    let difficulty = game.local_difficulty(request.position);
    spawning::finalize(&mut butterfly, &facts, &difficulty);
    raider::finalize(&mut butterfly, raider::Reason::Natural);

    let leader_index = pending.len().checked_sub(request.group_index as usize);
    let leader_palette = if request.group_index > 0 {
        leader_index
            .and_then(|index| pending.get(index))
            .filter(|leader| leader.kind.id == "naturalist:butterfly")
            .and_then(|leader| {
                leader
                    .ai
                    .nav
                    .__horizon_mixins_naturalist_butterfly_spawn_pool()
                    .as_ref()
            })
            .map(|(pair, _)| pair.clone())
    } else {
        None
    };
    let pair = leader_palette.unwrap_or_else(|| {
        std::array::from_fn(|_| {
            super::naturalist_deer_variants::select_group_variant(game, &mut butterfly)
                .unwrap_or_else(|| "naturalist:monarch".to_owned())
        })
    });
    let choice = butterfly.random.next_int_bound(2).unwrap_or(0) as usize;
    *butterfly
        .ai
        .nav
        .__horizon_mixins_naturalist_butterfly_spawn_pool_mut() = Some((pair, choice));
    let bounds = butterfly.bounds();
    pending.push(butterfly);
    ControlFlow::Break(Some(Spawned {
        bounds,
        school: true,
    }))
}
