fn deer_saved_love(id: &str) -> std::ops::ControlFlow<bool> {
    if matches!(
        id,
        "naturalist:alligator"
            | "naturalist:bear"
            | "naturalist:bird"
            | "naturalist:black_bear"
            | "naturalist:boar"
            | "naturalist:mole"
            | "naturalist:butterfly"
            | "naturalist:capybara"
            | "naturalist:caterpillar"
            | "naturalist:crab"
            | "naturalist:deer" | "naturalist:giraffe" | "naturalist:elephant" | "naturalist:mammoth" | "naturalist:zebra"
            | "naturalist:desert_scorpion"
            | "naturalist:duck"
            | "naturalist:firefly"
            | "naturalist:giant_isopod"
            | "naturalist:hippo"
            | "naturalist:snake"
            | "naturalist:rat"
            | "naturalist:tiger"
            | "naturalist:lion"
            | "naturalist:jungle_scorpion"
            | "naturalist:komodo_dragon"
            | "naturalist:lizard"
            | "naturalist:rhino"
            | "naturalist:ant"
            | "naturalist:hedgehog"
            | "naturalist:snail"
            | "naturalist:tortoise" | "naturalist:ostrich"
            | "naturalist:turkey"
    ) {
        std::ops::ControlFlow::Break(true)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
