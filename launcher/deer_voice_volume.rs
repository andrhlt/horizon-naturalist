fn deer_voice_volume(mob: &Mob) -> std::ops::ControlFlow<f32> {
    use std::ops::ControlFlow;
    match mob.kind.id.as_str() {
        "naturalist:ant" => ControlFlow::Break(1.0),
        "naturalist:snake" => ControlFlow::Break(0.15),
        "naturalist:desert_scorpion" => ControlFlow::Break(0.7),
        "naturalist:jungle_scorpion" => ControlFlow::Break(0.9),
        "naturalist:hedgehog" | "naturalist:deer" | "naturalist:giraffe" | "naturalist:elephant" | "naturalist:mammoth" | "naturalist:bear" | "naturalist:black_bear" | "naturalist:capybara" => ControlFlow::Break(1.0),
        _ => ControlFlow::Continue(()),
    }
}
