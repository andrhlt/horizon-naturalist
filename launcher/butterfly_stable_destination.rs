fn butterfly_stable_destination(m: &Mob, w: &MobWorld, pos: [i32; 3]) -> std::ops::ControlFlow<bool> {
    if m.kind.id == "naturalist:butterfly" {
        let [x, y, z] = pos;
        return std::ops::ControlFlow::Break(!w.facts.is_air(w.world.block(x, y - 1, z)));
    }
    std::ops::ControlFlow::Continue(())
}
