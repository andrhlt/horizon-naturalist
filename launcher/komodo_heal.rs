fn komodo_heal(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use std::ops::ControlFlow;
    let Some(i) = game.mob_interaction_target(target) else { return ControlFlow::Continue(()) };
    let mob = &game.mobs[i];
    if mob.kind.id != "naturalist:komodo_dragon" || !mob.alive() || mob.health >= mob.attrs.max_health
        || game.held().is_none_or(|stack| !mob.kind.food.iter().any(|food| food == stack.item.as_ref()))
    {
        return ControlFlow::Continue(());
    }
    game.mobs[i].heal(2.0);
    game.consume_held();
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
