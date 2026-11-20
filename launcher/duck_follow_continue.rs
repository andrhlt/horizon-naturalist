fn duck_follow_continue(goal: &FollowParent, mob: &Mob, context: &Cx<'_, '_>) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    let bear = super::naturalist_bears::is_bear(&mob.kind.id);
    if !bear && !matches!(mob.kind.id.as_str(), "naturalist:duck" | "naturalist:capybara") {
        return ControlFlow::Continue(());
    }
    let threshold = if bear { 144.0 } else { 25.0 };
    let following = mob.extra.age < 0 && !mob.extra.tame
        && goal.parent.and_then(|id| context.w.seen(id)).is_some_and(|parent| {
            parent.alive && (threshold..=256.0).contains(&mob.distance_sqr(parent.position))
        });
    ControlFlow::Break(following)
}
