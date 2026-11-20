fn shark_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::mobs::nav::naturalist_shark_state::KIND;
    use std::ops::ControlFlow;

    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let mob = &game.mobs[index];
    let (tag, healing) = match mob.kind.id.as_str() {
        KIND => ("naturalist:great_white_shark_food_items", 2.0),
        "naturalist:whale" => ("naturalist:whale_food_items", 4.0),
        _ => return ControlFlow::Continue(()),
    };
    if !mob.alive() || mob.health >= mob.attrs.max_health {
        return ControlFlow::Continue(());
    }
    let Some(stack) = game.held() else {
        return ControlFlow::Continue(());
    };
    if !game
        .items()
        .item_tags
        .get(tag)
        .is_some_and(|food| food.contains(stack.item.as_ref()))
    {
        return ControlFlow::Continue(());
    }
    if !mob.silent {
        game.audio.play(
            "minecraft:entity.generic.eat",
            Some(mob.position.components()),
            1.0,
            1.0,
        );
    }
    game.mobs[index].heal(healing);
    game.consume_held();
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
