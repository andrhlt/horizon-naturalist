fn crab_client_tame(
    game: &mut Game<'_>,
    index: usize,
    hand: Hand,
) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if hand != Hand::Main {
        return ControlFlow::Continue(());
    }
    let owner = Some(game.acting_player_uuid());
    let Some(mob) = game.mobs.get(index) else {
        return ControlFlow::Continue(());
    };
    if mob.kind.id != "naturalist:crab" || !mob.alive() {
        return ControlFlow::Continue(());
    }
    let tropical_fish = game
        .stack_in(game.selected)
        .is_some_and(|stack| stack.item.as_ref() == "minecraft:tropical_fish");
    if tropical_fish && (!mob.extra.tame || mob.extra.owner == owner
        && mob.health < mob.attrs.max_health)
    {
        return ControlFlow::Break(true);
    }
    ControlFlow::Continue(())
}
