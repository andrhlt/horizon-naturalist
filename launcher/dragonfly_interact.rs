fn dragonfly_interact(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use std::ops::ControlFlow;
    let Some(index) = game.mob_interaction_target(target) else { return ControlFlow::Continue(()) };
    let mob = &game.mobs[index];
    if mob.kind.id != "naturalist:dragonfly" || !mob.alive()
        || !game.held().is_some_and(|stack| stack.item.as_ref() == "minecraft:chorus_fruit")
    {
        return ControlFlow::Continue(());
    }
    let id = mob.id;
    let at = [mob.position.x, mob.position.y, mob.position.z];
    if !mob.silent {
        game.audio.play("minecraft:entity.generic.eat", Some(at), 1.0, 1.0);
    }
    game.consume_held();
    game.spawn_naturalist_dragonfly_cloud(id, at);
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
