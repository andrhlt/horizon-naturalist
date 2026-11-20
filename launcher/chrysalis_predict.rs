fn chrysalis_predict(
    game: &mut Game<'_>,
    hit: &super::Hit,
    stack: &ItemStack,
) -> std::ops::ControlFlow<Option<DryPlace>> {
    use horizon_core::{geometry::Direction, position::BlockPos};
    use horizon_world::{
        fluid::FluidState,
        level::{LevelAccess, PlacementContext},
    };
    use std::{cell::Cell, ops::ControlFlow};

    let sneaking = game.remote_shift_down();
    if game.may_build() && let Some((pos, state)) = super::naturalist_glow_goop::layer_state(
        game, hit, stack, sneaking,
    ) {
        let at = BlockPos::new(pos[0], pos[1], pos[2]);
        return ControlFlow::Break(Some((at, vec![(at, state)])));
    }

    thread_local! {
        static DRY_RUN: Cell<bool> = const { Cell::new(false) };
    }
    if stack.item.as_ref() != "naturalist:chrysalis" || DRY_RUN.replace(true) {
        return ControlFlow::Continue(());
    }
    let placed = game.remote_can_place(hit, stack);
    DRY_RUN.set(false);
    let Some((pos, mut changes)) = placed else {
        return ControlFlow::Break(None);
    };
    let hit_pos = BlockPos::new(hit.pos[0], hit.pos[1], hit.pos[2]);
    let context = PlacementContext {
        pos,
        hit_pos,
        face: Direction::ALL[face_direction(hit.face) as usize],
        hit: Vec3::new(hit.point[0], hit.point[1], hit.point[2]),
        inside: hit.face == [0; 3],
        yaw: game.yaw,
        pitch: game.pitch,
        sneaking: game.remote_shift_down(),
        item: stack.item.to_string(),
        replace_clicked: pos == hit_pos,
    };
    let Some((_, state)) = changes.iter_mut().find(|(changed, _)| *changed == pos) else {
        return ControlFlow::Break(None);
    };
    // The dry-run level uses the generic default for mod placement. Ask the
    // same authority placement hook about its resolved position and state.
    let mut answer = None;
    game.with_level(|_, level, _| {
        level.before_state_for_placement(*state, &context, FluidState::EMPTY, &mut answer);
    });
    let Some(Some(correct)) = answer else {
        return ControlFlow::Break(None);
    };
    *state = correct;
    ControlFlow::Break(Some((pos, changes)))
}
