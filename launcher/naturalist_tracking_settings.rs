fn naturalist_tracking_settings(kind: &str) -> std::ops::ControlFlow<(u8, u32, bool)> {
    use std::ops::ControlFlow;

    // Registered NeoForge entity ranges at Naturalist commit 1471124a.
    let range = match kind {
        "naturalist:bass"
        | "naturalist:catfish"
        | "naturalist:starfish" => 4,
        "naturalist:anglerfish"
        | "naturalist:ant"
        | "naturalist:bird"
        | "naturalist:blobfish"
        | "naturalist:butterfly"
        | "naturalist:carried_food"
        | "naturalist:clam"
        | "naturalist:crab"
        | "naturalist:desert_scorpion"
        | "naturalist:dirt_trail"
        | "naturalist:dragonfly"
        | "naturalist:firefly"
        | "naturalist:giant_isopod"
        | "naturalist:jellyfish"
        | "naturalist:piranha"
        | "naturalist:rat"
        | "naturalist:ray"
        | "naturalist:snake" => 8,
        "naturalist:alligator"
        | "naturalist:bear"
        | "naturalist:black_bear"
        | "naturalist:boar"
        | "naturalist:capybara"
        | "naturalist:caterpillar"
        | "naturalist:deer"
        | "naturalist:duck"
        | "naturalist:elephant"
        | "naturalist:giraffe"
        | "naturalist:great_white_shark"
        | "naturalist:hedgehog"
        | "naturalist:hippo"
        | "naturalist:jungle_scorpion"
        | "naturalist:komodo_dragon"
        | "naturalist:lion"
        | "naturalist:lizard"
        | "naturalist:lizard_tail"
        | "naturalist:mammoth"
        | "naturalist:mole"
        | "naturalist:ostrich"
        | "naturalist:rhino"
        | "naturalist:snail"
        | "naturalist:tiger"
        | "naturalist:tortoise"
        | "naturalist:turkey"
        | "naturalist:vulture"
        | "naturalist:whale"
        | "naturalist:zebra" => 10,
        "naturalist:duck_egg" => 16,
        _ => return ControlFlow::Continue(()),
    };
    ControlFlow::Break((range, 3, true))
}
