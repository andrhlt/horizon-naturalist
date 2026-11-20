fn tortoise_loaded_health(mob: &mut Mob) {
    use super::goals::naturalist_tortoise_goals as tortoise;

    // Tortoise.setTame runs after Java's saved attributes and Health are read.
    if mob.kind.id == tortoise::KIND {
        tortoise::tame_attributes(mob, true);
    }
}
