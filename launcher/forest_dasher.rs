fn forest_dasher(id: &str) -> std::ops::ControlFlow<Option<&'static PotionDef>> {
    use std::ops::ControlFlow;
    const POTIONS: &[PotionDef] = &[
        p("naturalist:antivenom", "antivenom", &[("naturalist:antivenom", 1, 0)]),
        p("naturalist:forest_dasher", "forest_dasher", &[("speed", 400, 1), ("weakness", 400, 0)]),
        p("naturalist:long_forest_dasher", "forest_dasher", &[("speed", 800, 1), ("weakness", 800, 0)]),
        p("naturalist:strong_forest_dasher", "forest_dasher", &[("speed", 400, 2), ("weakness", 400, 1)]),
    ];
    match POTIONS.iter().find(|potion| potion.id == id) {
        Some(potion) => ControlFlow::Break(Some(potion)),
        None => ControlFlow::Continue(()),
    }
}
