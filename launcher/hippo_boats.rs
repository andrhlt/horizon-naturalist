fn hippo_boats(game: &mut Game<'_>) {
    use horizon_client::mobs::nav::naturalist_hippo_state::{self as hippo, BoatTarget};
    if !game
        .mobs
        .iter()
        .any(|mob| mob.kind.id == hippo::KIND && mob.alive() && !mob.no_ai)
    {
        return;
    }
    let boats: std::sync::Arc<[BoatTarget]> = game
        .boats
        .iter()
        .filter(|boat| !boat.chest && boat.wood.as_ref() != "bamboo")
        .map(|boat| BoatTarget {
            id: boat.id,
            position: boat.position,
            bounds: boat.bounds(),
        })
        .collect();
    for mob in game
        .mobs
        .iter_mut()
        .filter(|mob| mob.kind.id == hippo::KIND && mob.alive() && !mob.no_ai)
    {
        hippo::state_mut(&mut mob.ai.nav).boats = boats.clone();
    }
}
