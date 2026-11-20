fn elephant_group(
    game: &Game<'_>,
    request: &SpawnRequest<'_>,
    pending: &mut Vec<Mob>,
) -> std::ops::ControlFlow<Option<Spawned>> {
    use horizon_client::mobs::{nav::naturalist_elephants, raider, spawning};
    use std::ops::ControlFlow;
    if !naturalist_elephants::includes(request.entity) {
        return ControlFlow::Continue(());
    }
    let Some(kind) = game.res.mob_types().get(request.entity).cloned() else {
        return ControlFlow::Break(None);
    };
    let [x, y, z] = request.position;
    let mut animal = Mob::new(0, kind, Vec3::new(x, y, z), request.yaw);
    let facts = game.biome_facts(x.floor() as i32, y.floor() as i32, z.floor() as i32);
    let difficulty = game.local_difficulty(request.position);
    spawning::finalize(&mut animal, &facts, &difficulty);
    raider::finalize(&mut animal, raider::Reason::Natural);
    if request.group_index >= 2 {
        horizon_client::mobs::finalize::set_age(&mut animal, -24_000);
    }
    let bounds = animal.bounds();
    pending.push(animal);
    ControlFlow::Break(Some(Spawned {
        bounds,
        school: true,
    }))
}
