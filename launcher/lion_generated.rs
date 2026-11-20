fn lion_generated(
    finalizer: &StructureMobFinalizer,
    mob: &mut Mob,
    random: &mut dyn PieceRandom,
    reason: Reason,
    group: &mut Option<Group>,
) -> std::ops::ControlFlow<Option<Compound>> {
    use crate::mobs::goals::naturalist_lion_goals as lion;
    use std::ops::ControlFlow;

    if mob.kind.id != lion::KIND || reason != Reason::ChunkGeneration {
        return ControlFlow::Continue(());
    }
    fn finalize_lion(mob: &mut Mob, random: &mut impl RandomSource, group: &mut Option<Group>) {
        use crate::mobs::{
            finalize::{self, Ageable},
            goals::naturalist_lion_goals as lion,
        };
        let _ = random.next_int_bound(1);
        let size = match group.as_ref() {
            Some(Group::Ageable(ages)) => ages.size.max(0),
            _ => 0,
        };
        let index = (size as u32 + 1) / 2;
        let level_cub = size != 0 && random.next_float() <= 0.05;
        finalize::mob_spawn(mob, random);
        lion::natural_pride_member(mob, index, level_cub);
        lion::apply_spawn_bonus(mob, random);
        let mut next = Ageable::babies(true);
        next.size = if size == 0 { 1 } else { size + 2 };
        *group = Some(Group::Ageable(next));
    }

    let any = random.as_any();
    if let Some(random) = any.downcast_mut::<LegacyRandom>() {
        finalize_lion(mob, random, group);
    } else if let Some(random) = any.downcast_mut::<XoroshiroRandom>() {
        finalize_lion(mob, random, group);
    } else {
        return ControlFlow::Continue(());
    }

    let raw = Tag::compound([
        ("Mane", Tag::Byte(i8::from(mob.extra.flag))),
        ("Variant", Tag::String(lion::KIND.into())),
    ]);
    ControlFlow::Break(Some(tree(
        &[(mob.clone(), Some(raw))],
        0,
        &finalizer.dimension,
    )))
}
