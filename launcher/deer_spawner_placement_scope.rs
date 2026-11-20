fn deer_spawner_placement_scope(
    game: &mut Game<'_>, key: Pos, spawner: &mut Spawner, cart: Option<u32>,
) -> std::ops::ControlFlow<Result<()>> {
    use super::naturalist_deer_placement::{Pass, PlacementPass};
    use horizon_client::mobs::naturalist_deer_spawn::{SpawnerRandom, finish_spawner};
    use std::ops::ControlFlow;

    let Some(_scope) = PlacementPass::enter(Pass::Spawner) else {
        return ControlFlow::Continue(());
    };
    let deer = "naturalist:deer";
    let current = spawner.next_data().and_then(SpawnDataExt::id);
    let needs_roll = current.is_none() || spawner.spawn_delay == -1;
    let selects_deer = if needs_roll && spawner.potentials.iter().any(|(data, _)| data.id().as_deref() == Some(deer)) {
        let mut copy = spawner.clone();
        let mut peek = game.world.random.clone();
        let mut events = Vec::new();
        if copy.spawn_delay == -1 {
            copy.delay(&mut peek, &mut events);
        }
        copy.next_or_create(&mut peek, &mut events);
        copy.spawn_delay <= 0 && copy.next_data().and_then(SpawnDataExt::id).as_deref() == Some(deer)
    } else {
        current.as_deref() == Some(deer)
    };
    let center = [key.0, key.1, key.2].map(|axis| f64::from(axis) + 0.5);
    if !selects_deer || spawner.spawn_delay > 0
        || !game.alive_player_near(center[0], center[1], center[2], f64::from(spawner.required_player_range))
        || !game.ui.rules.bool("spawner_blocks_work")
    {
        return ControlFlow::Break(game.server_tick_spawner_of(key, spawner, cart));
    }

    // The vanilla planner defers insertion until its immutable level view is dropped.
    // Finalize the accepted Deer before the spawner rolls its next potential.
    let old_random = std::mem::replace(
        &mut game.world.random,
        LevelRandom::for_level(LegacyRandom::new(0)),
    );
    let mut random = SpawnerRandom::new(old_random);
    let mut events = Vec::new();
    let mut planned = Vec::new();
    {
        let level = game.natural_level();
        let mut entity_random = random.clone();
        let mut spawn = |level: &super::natural_spawning::GameLevel, request: &SpawnerSpawn| {
            let result = level.game.plan_spawner_mob(level, request, &mut planned);
            if result.is_some() && request.id == deer && let Some(Planned::Finalized(mob, _)) = planned.last_mut() {
                let variant = super::naturalist_deer_variants::choose_spawn_variant(level.game, mob, &mut entity_random);
                *mob.ai.nav.__horizon_mixins_naturalist_deer_spawn_variant_mut() = variant;
                finish_spawner(mob, &mut entity_random);
            }
            result
        };
        spawner.server_tick(&level, &mut random, [key.0, key.1, key.2], &mut spawn, &mut events);
    }
    game.world.random = random.level_random();
    let ids = game.add_planned(planned);
    ControlFlow::Break(game.apply_spawner_events(key, events, &ids, cart))
}
