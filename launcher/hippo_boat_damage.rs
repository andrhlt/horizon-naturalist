fn hippo_boat_damage(
    game: &mut Game<'_>,
    target: u32,
    amount: f32,
    source: horizon_client::mobs::Damage,
) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::Who;
    use std::ops::ControlFlow;
    let Some(index) = game
        .boats
        .iter()
        .position(|boat| boat.id == target && !boat.chest && boat.wood.as_ref() != "bamboo")
    else {
        return ControlFlow::Continue(());
    };
    if !source.attacker.is_some_and(|who| {
        matches!(who, Who::Mob(id)
        if game.mobs.iter().any(|mob| mob.id == id && mob.kind.id == "naturalist:hippo"))
    }) {
        return ControlFlow::Continue(());
    }
    ControlFlow::Break(game.hurt_boat_result(index, amount, false).is_some())
}
