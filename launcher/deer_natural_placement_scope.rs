fn deer_natural_placement_scope(game: &mut Game<'_>) -> std::ops::ControlFlow<()> {
    use super::naturalist_deer_placement::{Pass, PlacementPass};
    use std::ops::ControlFlow;

    let Some(_scope) = PlacementPass::enter(Pass::Natural) else {
        return ControlFlow::Continue(());
    };
    horizon_client::mobs::spawn_selection::naturalist_aquatic_spawn_limits::natural_pass(|| game.spawn_natural());
    ControlFlow::Break(())
}
