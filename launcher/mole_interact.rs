fn mole_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::mobs::goals::naturalist_mole_goals::{self as mole, FOOD, MOLE, Phase};
    use std::ops::ControlFlow;
    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let animal = &game.mobs[index];
    if animal.kind.id != MOLE || !animal.alive() {
        return ControlFlow::Continue(());
    }
    if mole::phase(animal) != Phase::Unrolled {
        return ControlFlow::Break(Ok(false));
    }
    if animal.health >= animal.attrs.max_health || !game.held().is_some_and(|stack| stack.item.as_ref() == FOOD) {
        return ControlFlow::Continue(());
    }
    let at = animal.center();
    let position = animal.position;
    game.mobs[index].heal(2.0);
    game.consume_held();
    game.audio.play("minecraft:entity.generic.eat", Some(at), 1.0, 1.0);
    game.world.modified.insert((position.x.floor() as i32 >> 4, position.z.floor() as i32 >> 4));
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
