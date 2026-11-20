fn anglerfish_melee_visibility(ordinary: bool, mob: &Mob) -> bool {
    matches!(mob.kind.id.as_str(), "naturalist:anglerfish" | "naturalist:alligator")
        || super::nav::naturalist_elephants::includes(&mob.kind.id) || ordinary
}
