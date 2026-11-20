fn chrysalis_hatch(game: &mut Game<'_>) {
    let mut hatching = Vec::new();
    game.level_events.retain(|event| match event {
        LevelEvent::BlockEvent { pos, kind: "naturalist:chrysalis_hatch", .. } => {
            hatching.push(*pos);
            false
        }
        _ => true,
    });
    for pos in hatching {
        let at = [pos.x, pos.y, pos.z].map(|value| f64::from(value) + 0.5);
        if let Some(id) = game.spawn_mob_raw("naturalist:butterfly", at, false)
            && let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id)
        {
            mob.yaw = 0.0;
            mob.previous_yaw = 0.0;
            mob.body_yaw = 0.0;
            mob.head_yaw = 0.0;
        }
    }
}
