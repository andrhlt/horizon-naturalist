fn duck_portal(mob: &Mob) -> std::ops::ControlFlow<bool> {
    if matches!(mob.kind.id.as_str(), "naturalist:duck" | "naturalist:bass" | "naturalist:desert_scorpion" | "naturalist:rat") {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
