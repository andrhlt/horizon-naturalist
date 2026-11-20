fn scorpion_heal(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use std::ops::ControlFlow;
    if game.held().is_none_or(|stack| stack.item.as_ref() != "naturalist:lizard_tail") {
        return ControlFlow::Continue(());
    }
    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let mob = &mut game.mobs[index];
    if !matches!(mob.kind.id.as_str(), "naturalist:desert_scorpion" | "naturalist:jungle_scorpion")
        || !mob.alive() || mob.health >= mob.attrs.max_health
    {
        return ControlFlow::Continue(());
    }
    mob.heal(1.0);
    let at = mob.center();
    game.consume_held();
    game.audio.play("minecraft:entity.generic.eat", Some(at), 1.0, 1.0);
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
