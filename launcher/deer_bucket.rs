fn deer_bucket(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use std::ops::ControlFlow;
    if game.held().is_none_or(|stack| &*stack.item != "minecraft:bucket") {
        return ControlFlow::Continue(());
    }
    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    if !matches!(game.mobs[index].kind.id.as_str(), "naturalist:deer" | "naturalist:capybara" | "naturalist:desert_scorpion" | "naturalist:jungle_scorpion" | "naturalist:lizard" | "naturalist:lizard_tail" | "naturalist:komodo_dragon" | "naturalist:rhino" | "naturalist:bear" | "naturalist:black_bear") {
        return ControlFlow::Continue(());
    }
    if game.interact_leash_with_item_context(index, "minecraft:bucket").is_some() {
        let entity = game.mob_entity(&game.mobs[index]);
        let empty = ItemStack::new("minecraft:air", 0);
        game.trigger_item_entity("minecraft:player_interacted_with_entity", &empty, &entity);
        game.swing_arm();
        return ControlFlow::Break(Ok(true));
    }
    ControlFlow::Break(Ok(false))
}
