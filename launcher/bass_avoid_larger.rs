fn bass_avoid_larger(goal: &mut AvoidEntity, mob: &mut Mob, context: &mut Cx<'_, '_>) -> std::ops::ControlFlow<Option<Vec3>> {
    use super::nav::naturalist_bass_state::snapshot_tier;
    use std::ops::ControlFlow;

    if !*goal.__horizon_mixins_naturalist_bass_avoid_larger() {
        return ControlFlow::Continue(());
    }
    let tier = mob.extra.variant;
    let id = mob.id;
    let max = f64::from(goal.max_dist);
    let nearby = context.w.nearby(mob.position, max, move |seen| {
        seen.id != id && snapshot_tier(seen).is_some_and(|other| other > tier)
    });
    let threat = nearby.into_iter().find(|seen| mob.sees(context.w, Who::Mob(seen.id)));
    goal.found = threat.map(|seen| seen.id);
    ControlFlow::Break(threat.map(|seen| seen.position))
}
