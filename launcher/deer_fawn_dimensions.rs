fn deer_fawn_dimensions(mob: &Mob) -> std::ops::ControlFlow<(f64, f64, f64)> {
    use std::ops::ControlFlow;
    if !matches!(
        mob.kind.id.as_str(),
        "naturalist:deer" | "naturalist:giraffe" | "naturalist:elephant" | "naturalist:mammoth" | "naturalist:boar" | "naturalist:turkey" | "naturalist:capybara"
            | "naturalist:lizard" | "naturalist:desert_scorpion" | "naturalist:jungle_scorpion"
            | "naturalist:komodo_dragon" | "naturalist:rhino" | "naturalist:bear" | "naturalist:black_bear"
            | "naturalist:hedgehog" | "naturalist:mole" | "naturalist:whale" | "naturalist:great_white_shark" | "naturalist:hippo" | "naturalist:ant" | "naturalist:alligator" | "naturalist:duck" | "naturalist:giant_isopod" | "naturalist:crab" | "naturalist:firefly" | "naturalist:snail" | "naturalist:zebra" | "naturalist:tortoise" | "naturalist:tiger" | "naturalist:lion" | "naturalist:snake" | "naturalist:rat" | "naturalist:ostrich"
    ) || !mob.is_baby() {
        return ControlFlow::Continue(());
    }
    let scale = mob.model_scale();
    let scaled = |dimension: f64| f64::from(dimension as f32 * 0.5 * scale);
    let kind = &mob.kind;
    ControlFlow::Break((scaled(kind.width), scaled(kind.height), scaled(kind.eye_height)))
}
