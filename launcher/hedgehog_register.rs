fn hedgehog_register(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    use super::{goals::naturalist_hedgehog_goals, naturalist_hedgehog::KIND};
    use std::ops::ControlFlow;
    if mob.kind.id != KIND {
        return ControlFlow::Continue(());
    }
    mob.extra.color = u8::MAX;
    mob.ai.goals.add(1, SitWhenOrdered);
    naturalist_hedgehog_goals::register(mob);
    ControlFlow::Break(())
}
