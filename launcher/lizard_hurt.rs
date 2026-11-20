fn lizard_hurt(mob: &mut Mob) {
    use naturalist_lizard::{mark_tail_dropped, missing_tail};
    if mob.kind.id == "naturalist:lizard" && mob.alive()
        && mob.health <= 0.5 * mob.attrs.max_health && !missing_tail(&mob.ai.nav)
    {
        mark_tail_dropped(&mut mob.ai.nav);
    }
}
