fn ant_ai_step(mob: &mut Mob, world: &MobWorld) -> std::ops::ControlFlow<()> {
    use super::naturalist_ant::{ANGER_TICKS, ANT, state_mut};
    use std::ops::ControlFlow;
    if mob.kind.id != ANT {
        return ControlFlow::Continue(());
    }
    if mob.ai.target == Some(Who::Player) && is_owner(mob, world) {
        stop_being_angry(mob);
    }
    match mob.ai.target {
        Some(Who::Player) if !valid_player_target(world) => update_persistent_anger(mob, world),
        Some(Who::Mob(id)) if world.seen(id).is_none_or(|seen| !seen.alive) => {
            update_persistent_anger(mob, world);
        }
        Some(target) => {
            mob.extra.angry_at = Some(target);
            mob.extra.anger = ANGER_TICKS;
        }
        None => update_persistent_anger(mob, world),
    }
    let state = state_mut(mob);
    state.cooldown = state.cooldown.saturating_sub(1).max(0);
    ControlFlow::Break(())
}
