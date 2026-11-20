fn jellyfish_goals(mob: &mut Mob) {
    if mob.kind.id == "naturalist:jellyfish" {
        mob.ai.goals.clear();
        mob.ai.goals.add(0, super::goals::Stroll::swimming(1.0, 20));
    }
}
