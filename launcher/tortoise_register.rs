fn tortoise_register(mob: &mut Mob) {
    use super::goals::naturalist_tortoise_goals as tortoise;
    if mob.kind.id == tortoise::KIND {
        tortoise::register(mob);
    }
}
