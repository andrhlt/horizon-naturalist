fn tortoise_dig(mob: &Mob, context: &mut Cx<'_, '_>) {
    use super::goals::naturalist_tortoise_goals as tortoise;
    if mob.kind.id == tortoise::KIND {
        tortoise::dig_effect(mob, context);
    }
}
