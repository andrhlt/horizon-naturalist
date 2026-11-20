fn vulture_ai_tick(keep: bool, mob: &mut Mob) -> bool {
    if keep && mob.kind.id == "naturalist:vulture" {
        super::goals::naturalist_vulture_goals::tick(mob);
    }
    keep
}
