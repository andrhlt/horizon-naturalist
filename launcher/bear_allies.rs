fn bear_allies(mob: &Mob, world: &MobWorld, target: Who) -> std::ops::ControlFlow<bool> {
    if (matches!(mob.kind.id.as_str(), "naturalist:hippo" | "naturalist:tiger" | "naturalist:lion" | "naturalist:snake") || super::naturalist_bears::is_bear(&mob.kind.id) || super::nav::naturalist_elephants::includes(&mob.kind.id)) && mob.extra.tame
        && let Some(owner) = mob.extra.owner
        && let Who::Mob(id) = target
        && world.seen(id).is_some_and(|other| (other.tame || matches!(mob.kind.id.as_str(), "naturalist:tiger" | "naturalist:lion" | "naturalist:snake")) && other.owner == Some(owner))
    {
        return std::ops::ControlFlow::Break(true);
    }
    std::ops::ControlFlow::Continue(())
}
