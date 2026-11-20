fn vulture_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use std::ops::ControlFlow;
    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let mob = &game.mobs[index];
    if mob.kind.id != "naturalist:vulture" {
        return ControlFlow::Continue(());
    }
    if let Some(item) = game.held()
        && (mob.kind.food.iter().any(|food| food == item.item.as_ref())
            || game
                .items()
                .item_in_tag("minecraft:parrot_poisonous_food", &item.item))
    {
        return ControlFlow::Break(Ok(false));
    }
    ControlFlow::Continue(())
}
