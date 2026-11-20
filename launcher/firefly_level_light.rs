fn firefly_level_light(game: &mut Game<'_>) {
    use horizon_client::mobs::goals::naturalist_firefly_behavior::set_level_light;

    let sky_darken = game.sky_darken();
    let dimension = game.dimension_type();
    let (fixed_time, ambient) = (dimension.has_fixed_time, dimension.ambient_light);
    for mob in game.mobs.iter_mut().filter(|mob| mob.kind.id == "naturalist:firefly") {
        set_level_light(mob, sky_darken, fixed_time, ambient);
    }
}
