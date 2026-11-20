fn bear_tamable(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if matches!(mob.kind.id.as_str(), "naturalist:hippo" | "naturalist:tiger" | "naturalist:lion" | "naturalist:snake" | "naturalist:rat") || super::naturalist_bears::is_bear(&mob.kind.id) || super::nav::naturalist_elephants::includes(&mob.kind.id) {
        return std::ops::ControlFlow::Break(true);
    }
    std::ops::ControlFlow::Continue(())
}
