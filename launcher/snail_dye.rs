fn snail_dye(
    game: &mut Game<'_>,
    target: Option<InteractTarget>,
) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::mobs::naturalist_snail::{snail, snail_mut};
    use horizon_client::mobs::render::DYE_NAMES;
    use std::ops::ControlFlow;

    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let mob = &game.mobs[index];
    if mob.kind.id != "naturalist:snail" || !mob.alive() {
        return ControlFlow::Continue(());
    }
    let Some(dye) = game
        .held()
        .and_then(|stack| stack.item.strip_prefix("minecraft:"))
        .and_then(|name| name.strip_suffix("_dye"))
        .and_then(|dye| DYE_NAMES.iter().position(|name| *name == dye))
    else {
        return ControlFlow::Continue(());
    };
    let dye = dye as u8;
    if snail(mob).color == dye {
        return ControlFlow::Continue(());
    }
    snail_mut(&mut game.mobs[index]).color = dye;
    game.consume_held();
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
