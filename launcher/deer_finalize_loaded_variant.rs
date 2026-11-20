fn deer_finalize_loaded_variant(
    difficulty: horizon_client::mobs::spawning::LocalDifficulty,
    game: &mut Game<'_>,
    mob: &mut horizon_client::mobs::Mob,
    tag: &mut Tag,
) -> horizon_client::mobs::spawning::LocalDifficulty {
    if let Some(id) = super::naturalist_deer_variants::select_variant(game, &mob.kind.id, mob.position)
        && let Tag::Compound(fields) = tag
    {
        fields.insert("Variant".into(), Tag::String(id.as_str().into()));
    }
    if mob.kind.id == "naturalist:deer" {
        horizon_client::mobs::naturalist_deer_spawn::finish(mob, &mut game.world.random);
    } else {
        horizon_client::mobs::nav::naturalist_elephants::add_spawn_bonus(mob, &mut game.world.random);
    }
    difficulty
}
