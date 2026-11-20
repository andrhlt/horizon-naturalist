fn baby_health_save(
    map: &mut Compound, mob: &Mob, attributes: &crate::mobs::finalize::Attributes,
) -> std::ops::ControlFlow<()> {
    use crate::mobs::{finalize::Attr, nav::naturalist_baby_health::{MODIFIER_ID, has_baby_modifier_lifecycle}};
    use std::ops::ControlFlow;

    if !has_baby_modifier_lifecycle(&mob.ai.nav)
        || !attributes.modifiers.iter().any(|modifier| modifier.attr == Attr::MaxHealth && modifier.id == MODIFIER_ID) {
        return ControlFlow::Continue(());
    }
    let mut saved = attributes.clone();
    saved.modifiers.retain(|modifier| modifier.attr != Attr::MaxHealth || modifier.id != MODIFIER_ID);
    put_attributes(map, mob, &saved);
    ControlFlow::Break(())
}
