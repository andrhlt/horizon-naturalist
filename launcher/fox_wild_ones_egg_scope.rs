fn fox_wild_ones_egg_scope(game: &mut Game<'_>, stack: &ItemStack) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::mobs::fox::naturalist_wild_ones::PlayerEgg;
    use std::ops::ControlFlow;

    if stack.item.as_ref() != "minecraft:fox_spawn_egg" {
        return ControlFlow::Continue(());
    }
    // Only this player's egg path enters the shared spawn method in this
    // scope. Commands and dispensers must keep their ordinary Fox equipment.
    let Some(_scope) = PlayerEgg::enter() else { return ControlFlow::Continue(()) };
    ControlFlow::Break(game.use_spawn_egg(stack))
}
