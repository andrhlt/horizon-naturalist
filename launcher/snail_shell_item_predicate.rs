fn snail_shell_item_predicate(
    game: &Game<'_>,
    predicate: &ItemInput,
    stack: &ItemStack,
) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    const COMPONENT: &str = "minecraft:custom_data";
    if stack.item.as_ref() != "naturalist:snail_shell"
        || predicate.components.is_none()
        || stack.data().is_some_and(|data| {
            data.other(COMPONENT).is_some() || data.other("!minecraft:custom_data").is_some()
        })
    {
        return ControlFlow::Continue(());
    }

    let view = horizon_client::item_render::stack_view(stack);
    let effective = game.pack.item_renderer.models.components_view(&view);
    let Some(data) = effective.get(COMPONENT) else {
        return ControlFlow::Continue(());
    };
    // The stock predicate reads serialized changes. Expose just the Shell
    // prototype on a temporary stack, leaving the authoritative stack alone.
    let mut comparison = stack.clone();
    comparison.data_mut().set_other(
        COMPONENT,
        Some(horizon_client_core::scoreboard::json_to_tag(data)),
    );
    ControlFlow::Break(game.stack_matches(predicate, &comparison))
}
