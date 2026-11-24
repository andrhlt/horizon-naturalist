fn scorpion_tail_interact(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    let Some(index) = game.mob_interaction_target(target) else { return std::ops::ControlFlow::Continue(()); };
    let kind = game.mobs[index].kind.id.as_str();
    if !matches!(kind, "naturalist:desert_scorpion" | "naturalist:jungle_scorpion") || !game.mobs[index].alive() { return std::ops::ControlFlow::Continue(()); }
    if game.held().map(|stack| stack.item.as_ref()) != Some("naturalist:lizard_tail") || game.mobs[index].health >= game.mobs[index].attrs.max_health { return std::ops::ControlFlow::Continue(()); }
    if !game.creative { game.consume_held(); }
    game.mobs[index].heal(&game.loop_context, 1.0);
    std::ops::ControlFlow::Break(Ok(true))
}
