fn frog_food(_result: (), mob: &mut Mob, world: &MobWorld<'_>) {
    use super::nav::naturalist_frog_food::FOOD;

    let frog = &mob.extra.frog;
    if frog.scan != SCAN_RATE {
        return;
    }
    let vanilla_target = frog.nearest;
    let eye = Vec3::new(mob.position.x, mob.eye_y(), mob.position.z);
    let nearest = world
        .nearby(mob.position, DETECTION_DISTANCE, |candidate| {
            if Some(candidate.id) == vanilla_target {
                return true;
            }
            candidate.effects.contains(&FOOD)
                && (!matches!(candidate.species, Species::Slime | Species::MagmaCube)
                    || candidate.size == 1)
                && !frog.unreachable.iter().any(|(id, _)| *id == candidate.id)
                && world.line_of_sight(
                    eye,
                    Vec3::new(candidate.position.x, candidate.eye_y, candidate.position.z),
                )
        })
        .find(|candidate| {
            candidate.position.distance_squared(mob.position) < DETECTION_DISTANCE * DETECTION_DISTANCE
        })
        .map(|candidate| candidate.id);
    mob.extra.frog.nearest = nearest;
}
