fn naturalist_mob_apply_nbt(
    game: &mut Game<'_>, target: Target, tag: &Tag,
) -> std::ops::ControlFlow<R<()>> {
    use crate::game::naturalist_deer_scale as scale;
    use horizon_client::mobs::{
        finalize::{self, Attr, Op},
        nav::naturalist_baby_health::{self, MODIFIER_ID},
    };
    use std::{cell::Cell, ops::ControlFlow};
    thread_local! { static REENTER: Cell<bool> = const { Cell::new(false) }; }

    if REENTER.replace(false) {
        return ControlFlow::Continue(());
    }
    let Target::Mob(id) = target else { return ControlFlow::Continue(()) };
    let Some(mob) = game.mobs.iter().find(|mob| mob.id == id) else {
        return ControlFlow::Continue(());
    };
    let is_naturalist = mob.kind.id.starts_with("naturalist:");
    let baby_modifier = naturalist_baby_health::has_baby_modifier_lifecycle(&mob.ai.nav)
        .then(|| finalize::modifier_amount(mob, Attr::MaxHealth, MODIFIER_ID))
        .flatten();
    let packed_scale = is_naturalist.then(|| scale::read(tag)).flatten();
    let restore_scale = is_naturalist
        && (packed_scale.is_some() || game.cmds.attributes.contains_key(&target));
    if !restore_scale && baby_modifier.is_none() {
        return ControlFlow::Continue(());
    }

    REENTER.set(true);
    let result = game.apply_entity_nbt(target, tag.clone());
    if result.is_ok() {
        if restore_scale {
            scale::restore(game, id, packed_scale);
        }
        if let Some(amount) = baby_modifier
            && let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id)
            && !finalize::has_modifier(mob, Attr::MaxHealth, MODIFIER_ID)
        {
            finalize::add_modifier(mob, Attr::MaxHealth, MODIFIER_ID, amount, Op::AddMultipliedTotal);
            mob.health = mob.health.min(mob.attrs.max_health);
        }
    }
    ControlFlow::Break(result)
}
