fn ant_sugar(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use mod_api::{Host, SoundSource};
    use std::ops::ControlFlow;

    let Some(mut index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    if game.mobs[index].kind.id != "naturalist:ant" || !game.mobs[index].alive() {
        return ControlFlow::Continue(());
    }
    // These generic Mob interactions precede species-specific behavior in the
    // pinned game. Ant extends TamableAnimal and must not run Wolf's branches.
    let used = game.held().cloned();
    if game.name_mob(index) {
        if let Some(stack) = &used {
            let entity = game.mob_entity(&game.mobs[index]);
            game.trigger_item_entity("minecraft:player_interacted_with_entity", stack, &entity);
        }
        return ControlFlow::Break(Ok(true));
    }
    if game.held_shears()
        && let Some(consumed) = game.player_shear(&mut index)
    {
        return ControlFlow::Break(Ok(consumed));
    }
    if game.spawn_egg_offspring(index) {
        game.swing_arm();
        return ControlFlow::Break(Ok(true));
    }
    let item = game
        .held()
        .map(|stack| stack.item.to_string())
        .unwrap_or_default();
    if let Some(context) = game.interact_leash_with_item_context(index, &item) {
        let used = match context {
            crate::game::leads::LeashInteraction::WithItem => used,
            crate::game::leads::LeashInteraction::WithoutItem => {
                Some(horizon_client::items::ItemStack::new("minecraft:air", 0))
            }
        };
        if let Some(stack) = &used {
            let entity = game.mob_entity(&game.mobs[index]);
            game.trigger_item_entity("minecraft:player_interacted_with_entity", stack, &entity);
        }
        game.swing_arm();
        return ControlFlow::Break(Ok(true));
    }
    if item != "minecraft:sugar" || game.mobs[index].health >= game.mobs[index].attrs.max_health {
        return ControlFlow::Break(Ok(false));
    }
    let at = game.mobs[index].center();
    game.mobs[index].heal(&game.loop_context, 1.0);
    if let Some(level) = game.level(game.dim.id()) {
        level.play_sound(
            game,
            "minecraft:entity.generic.eat",
            at,
            SoundSource::Neutral,
            1.0,
            1.0,
        );
    }
    game.consume_held();
    ControlFlow::Break(Ok(true))
}
