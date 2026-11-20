fn tortoise_summoned(result: Option<u32>, game: &mut Game<'_>, kind: &str, yaw: SpawnYaw) -> Option<u32> {
    use horizon_client::mobs::goals::naturalist_tortoise_goals as tortoise;
    if kind == tortoise::KIND && matches!(yaw, SpawnYaw::Fixed(_))
        && let Some(mob) = result.and_then(|id| game.mobs.iter_mut().find(|mob| mob.id == id))
    {
        tortoise::tame_attributes(mob, false);
    }
    result
}
