fn deer_step(
    mob: &Mob,
    world: &MobWorld<'_>,
    state: horizon_world::registry::BlockStateId,
) -> std::ops::ControlFlow<Option<(&'static str, f32, f32)>> {
    if mob.kind.id == "naturalist:ant" {
        return std::ops::ControlFlow::Break(Some(("naturalist:entity.ant.step", 0.375, 1.0)));
    }
    if mob.kind.id == "naturalist:rat" {
        return std::ops::ControlFlow::Break(None);
    }
    if !matches!(
        mob.kind.id.as_str(),
        "naturalist:hedgehog" | "naturalist:deer"
            | "naturalist:elephant"
            | "naturalist:mammoth"
            | "naturalist:bird"
            | "naturalist:firefly"
            | "naturalist:giant_isopod"
            | "naturalist:crab"
            | "naturalist:capybara"
            | "naturalist:turkey"
            | "naturalist:tortoise" | "naturalist:tiger" | "naturalist:lion" | "naturalist:snake" | "naturalist:ostrich"
            | "naturalist:lizard"
            | "naturalist:lizard_tail"
            | "naturalist:komodo_dragon"
            | "naturalist:desert_scorpion"
            | "naturalist:jungle_scorpion"
            | "naturalist:rhino"
            | "naturalist:giraffe"
    ) {
        return std::ops::ControlFlow::Continue(());
    }
    let sounds = &world.facts.get(state).sounds;
    std::ops::ControlFlow::Break(Some((
        intern_sound(&sounds.step),
        sounds.volume * 0.15,
        sounds.pitch,
    )))
}
