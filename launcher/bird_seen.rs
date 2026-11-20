fn bird_seen(mut seen: Seen, mob: &Mob) -> Seen {
    if mob.kind.id == "naturalist:bird" {
        seen.variant = super::nav::naturalist_bird_navigation::BIRD_SNAPSHOT;
        // The stock snapshot reserves these stats for horse breeding.
        seen.stats = mob.ai.look.wanted;
    }
    seen
}
