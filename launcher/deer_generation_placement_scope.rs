fn deer_generation_placement_scope(game: &mut Game<'_>, cx: i32, cz: i32) -> std::ops::ControlFlow<()> {
    use super::naturalist_deer_placement::{Pass, PlacementPass};
    use std::ops::ControlFlow;

    let Some(_scope) = PlacementPass::enter(Pass::Generation) else {
        return ControlFlow::Continue(());
    };
    let files = horizon_client::mobs::naturalist_spawn_variants::generation_files(game.pack_files());
    horizon_client::mobs::naturalist_spawn_variants::with_generation_files(files, || {
        game.spawn_generation_creatures(cx, cz);
    });
    ControlFlow::Break(())
}
