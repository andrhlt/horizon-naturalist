fn duck_step(mob: &Mob) -> std::ops::ControlFlow<Option<(&'static str, f32, f32)>> {
    use std::ops::ControlFlow;
    match mob.kind.id.as_str() {
        "naturalist:bear" | "naturalist:black_bear" => ControlFlow::Break(Some(("entity.polar_bear.step", 0.15, 1.0))),
        "naturalist:boar" => ControlFlow::Break(Some(("entity.pig.step", 0.15, 1.0))),
        "naturalist:duck" => ControlFlow::Break(Some(("naturalist:entity.duck.step", 0.1, 1.2))),
        _ => ControlFlow::Continue(()),
    }
}
