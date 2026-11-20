fn deer_voice_pitch(mob: &mut Mob) -> std::ops::ControlFlow<f32> {
    if naturalist_bears::is_bear(&mob.kind.id) {
        let pitch = 1.0 + (mob.random.next_float() - mob.random.next_float()) * 0.2;
        return std::ops::ControlFlow::Break(if naturalist_bears::state(&mob.ai.nav).sleeping { pitch * 0.3 } else { pitch });
    }
    if mob.kind.id == "naturalist:jungle_scorpion" {
        let base = if mob.is_baby() { 1.5 } else { 1.0 };
        return std::ops::ControlFlow::Break(
            ((mob.random.next_float() - mob.random.next_float()) * 0.2 + base) * 0.75,
        );
    }
    if mob.kind.id == "naturalist:boar" {
        let base = if mob.is_baby() { 1.0 } else { 0.5 };
        return std::ops::ControlFlow::Break(
            (mob.random.next_float() - mob.random.next_float()) * 0.2 + base,
        );
    }
    if matches!(mob.kind.id.as_str(), "naturalist:ostrich" | "naturalist:hedgehog" | "naturalist:deer" | "naturalist:giraffe" | "naturalist:elephant" | "naturalist:mammoth" | "naturalist:duck" | "naturalist:capybara" | "naturalist:rhino" | "naturalist:alligator" | "naturalist:tortoise" | "naturalist:lion") {
        return std::ops::ControlFlow::Break(
            (mob.random.next_float() - mob.random.next_float()) * 0.2 + 1.0,
        );
    }
    std::ops::ControlFlow::Continue(())
}
