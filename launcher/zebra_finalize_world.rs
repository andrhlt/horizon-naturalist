fn zebra_finalize_world(
    finalizer: &StructureMobFinalizer, mob: &mut Mob, raw: &Option<Tag>,
    random: &mut dyn PieceRandom, group: &mut Option<Group>,
) -> std::ops::ControlFlow<Option<Compound>> {
    use crate::mobs::naturalist_zebra as zebra;
    if mob.kind.id != zebra::KIND {
        return std::ops::ControlFlow::Continue(());
    }
    let random = random.as_any();
    if let Some(random) = random.downcast_mut::<LegacyRandom>() {
        zebra::finish_generated(mob, random, group);
    } else if let Some(random) = random.downcast_mut::<XoroshiroRandom>() {
        zebra::finish_generated(mob, random, group);
    } else {
        return std::ops::ControlFlow::Continue(());
    }
    std::ops::ControlFlow::Break(Some(tree(&[(mob.clone(), raw.clone())], 0, &finalizer.dimension)))
}
