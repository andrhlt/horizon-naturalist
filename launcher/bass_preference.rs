fn bass_preference(goal: &mut NearestTarget, mob: &Mob) {
    use super::nav::naturalist_bass_state::SMALLEST_SNAPSHOT;
    if mob.kind.id != "naturalist:bass" || goal.label != Some("NaturalistBassHunt") {
        return;
    }
    goal.prey = Prey::Filter(if mob.extra.variant >= 2 {
        |seen| seen.species == Species::Cod && (SMALLEST_SNAPSHOT..SMALLEST_SNAPSHOT + 2).contains(&seen.variant)
    } else {
        |seen| seen.species == Species::Cod && seen.variant == SMALLEST_SNAPSHOT
    });
}
