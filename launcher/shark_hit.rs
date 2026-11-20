fn shark_hit(game: &Game<'_>, reach: f64) -> std::ops::ControlFlow<Option<(usize, f64, [f64; 3])>> {
    use horizon_client::mobs::nav::{naturalist_shark_state as shark, naturalist_whale_state as whale};
    use std::ops::ControlFlow;
    let Some((eye, direction)) = game.aim() else {
        return ControlFlow::Continue(());
    };
    let pickable = |mob: &Mob| {
        !mob.removed && Some(mob.id) != game.riding && Some(mob.id) != game.remote_camera_id()
    };
    let part = game
        .mobs
        .iter()
        .enumerate()
        .filter(|(_, mob)| matches!(mob.kind.id.as_str(), shark::KIND | whale::KIND) && pickable(mob))
        .filter_map(|(index, mob)| {
            let hit = |bounds| ray_box(eye, direction, reach, bounds);
            let distance = if mob.kind.id == shark::KIND {
                game.shark_part_boxes(mob).into_iter().filter_map(hit).min_by(f64::total_cmp)
            } else {
                game.whale_part_boxes(mob).into_iter().filter_map(hit).min_by(f64::total_cmp)
            }?;
            Some((index, distance, mob.position))
        })
        .min_by(|left, right| left.1.total_cmp(&right.1));
    let Some((index, distance, position)) = part else {
        return ControlFlow::Continue(());
    };
    if game.mobs.iter().any(|mob| {
        pickable(mob)
            && ray_box(eye, direction, reach, mob.bounds())
                .is_some_and(|ordinary| ordinary <= distance)
    }) {
        return ControlFlow::Continue(());
    }

    let point = Vec3::new(
        eye[0] + direction[0] * distance,
        eye[1] + direction[1] * distance,
        eye[2] + direction[2] * distance,
    );
    ControlFlow::Break(Some((index, distance, interact_location(point, position))))
}
