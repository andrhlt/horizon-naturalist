fn crab_revenge(mob: &Mob, context: &Cx<'_, '_>) -> std::ops::ControlFlow<bool> {
    use super::nav::naturalist_crab as crab;
    use std::ops::ControlFlow;

    if mob.kind.id != crab::KIND {
        return ControlFlow::Continue(());
    }
    if !crab::holding_weapon(mob) {
        return ControlFlow::Break(false);
    }
    if mob.extra.tame && mob.ai.last_hurt_by == Some(super::Who::Player)
        && context.w.player.as_ref().is_some_and(|player| mob.extra.owner == Some(player.uuid))
    {
        return ControlFlow::Break(false);
    }
    ControlFlow::Continue(())
}
