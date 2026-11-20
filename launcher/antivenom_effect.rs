fn antivenom_effect(id: &str) -> std::ops::ControlFlow<Option<&'static EffectDef>> {
    const ANTIVENOM: EffectDef = EffectDef {
        id: "naturalist:antivenom",
        color: 0x6FD6B2,
        category: Category::Beneficial,
        instant: true,
    };
    if id == ANTIVENOM.id {
        std::ops::ControlFlow::Break(Some(&ANTIVENOM))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
