fn frog_firefly_glow(mob: &mut Mob, context: &Cx<'_, '_>) {
    use super::nav::naturalist_frog_food::FIREFLY;

    if mob.extra.frog.target.and_then(|id| context.w.seen(id))
        .is_some_and(|seen| seen.effects.contains(&FIREFLY))
    {
        mob.add_effect("minecraft:glowing", 60, 0);
    }
}
