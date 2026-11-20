fn starfish_goals(mob: &mut Mob) {
    if matches!(mob.kind.id.as_str(), "naturalist:starfish" | "naturalist:clam") {
        mob.ai.goals.clear();
        mob.ai.targets.clear();
    }
}
