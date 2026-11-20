fn deer_finalize_variant(
    difficulty: horizon_client::mobs::spawning::LocalDifficulty,
    game: &mut Game<'_>,
    mob: &mut Mob,
) -> horizon_client::mobs::spawning::LocalDifficulty {
    if let Some(id) = super::naturalist_deer_variants::select_variant(game, &mob.kind.id, mob.position) {
        super::naturalist_deer_variants::put_variant(game, mob.id, &id);
    }
    if mob.kind.id == "naturalist:deer" {
        horizon_client::mobs::naturalist_deer_spawn::finish(mob, &mut game.world.random);
    } else {
        horizon_client::mobs::nav::naturalist_elephants::add_spawn_bonus(mob, &mut game.world.random);
    }
    difficulty
}
