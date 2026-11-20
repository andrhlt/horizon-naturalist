fn piranha_tick(mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) {
    use super::nav::naturalist_piranha_state as piranha;
    if !matches!(mob.kind.id.as_str(), "naturalist:piranha" | "naturalist:ray" | "naturalist:anglerfish") || !mob.alive() || mob.removed {
        return;
    }
    let leader = mob.extra.leader.and_then(|id| world.seen(id)).filter(|seen| {
        seen.alive && seen.species == Species::Cod && seen.variant == piranha::SNAPSHOT
    });
    let target = piranha::take_retaliation(&mut mob.ai.nav);
    let accepts = |target: Option<Who>| target != Some(Who::Player) || world.difficulty.effective != 0.0;
    if let Some(attacker) = target.filter(|who| accepts(Some(*who))) {
        mob.ai.target = Some(attacker);
        if let Some(leader) = leader {
            events.push(MobEvent::SetTarget { mob: leader.id, target: attacker });
        }
    } else if let Some(leader) = leader && accepts(leader.target) {
        mob.ai.target = leader.target;
    }
}
