fn zebra_tame_tick(mob: &mut Mob, context: &mut Cx<'_, '_>) {
    use super::naturalist_zebra as zebra;
    if mob.kind.id == zebra::KIND {
        zebra::tame_tick(mob, context.events);
    }
}
