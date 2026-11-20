fn boar_saddle(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use std::ops::ControlFlow;
    let Some(held) = game.held().filter(|stack| stack.item.as_ref() == "minecraft:saddle").cloned() else {
        return ControlFlow::Continue(());
    };
    let Some(index) = game.mob_interaction_target(target).filter(|&index| {
        game.mobs[index].kind.id == "naturalist:boar"
    }) else {
        return ControlFlow::Continue(());
    };
    if let Some(context) = game.interact_leash_with_item_context(index, &held.item) {
        let item = match context {
            super::leads::LeashInteraction::WithItem => held,
            super::leads::LeashInteraction::WithoutItem => ItemStack::new("minecraft:air", 0),
        };
        let entity = game.mob_entity(&game.mobs[index]);
        game.trigger_item_entity("minecraft:player_interacted_with_entity", &item, &entity);
        game.swing_arm();
        return ControlFlow::Break(Ok(true));
    }
    ControlFlow::Break(Ok(false))
}
