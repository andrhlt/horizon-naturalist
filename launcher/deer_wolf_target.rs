fn deer_wolf_target(_result: (), wolf: &mut Mob) {
    fn deer(candidate: &super::Seen) -> bool {
        candidate.species == Species::Cow
            && super::naturalist_deer_social::animal_marker("naturalist:deer") == Some(candidate.variant)
    }

    if wolf.kind.id == "minecraft:wolf" {
        use super::goals::naturalist_mole_goals::{MOLE, WolfDig};
        wolf.ai.goals.add(3, WolfDig::default());
        wolf.ai.targets.add(4, NonTameTarget::new(NearestTarget::new(
            Prey::Filter(|candidate| candidate.effects.contains(&MOLE)), false,
        )));
        wolf.ai.targets.add(4, NonTameTarget::new(NearestTarget::new(Prey::Filter(deer), false)));
    }
}
