fn deer_shadow(mob: &Mob) -> std::ops::ControlFlow<f32> {
    if matches!(mob.kind.id.as_str(), "naturalist:bass" | "naturalist:catfish" | "naturalist:piranha" | "naturalist:blobfish" | "naturalist:jellyfish" | "naturalist:ray" | "naturalist:anglerfish" | "naturalist:starfish") { return std::ops::ControlFlow::Break(0.0); }
    if matches!(mob.kind.id.as_str(), "naturalist:duck" | "naturalist:bird") {
        return std::ops::ControlFlow::Break(if mob.is_baby() { 0.15 } else { 0.3 });
    }
    if matches!(mob.kind.id.as_str(), "naturalist:lizard" | "naturalist:lizard_tail" | "naturalist:desert_scorpion") { return std::ops::ControlFlow::Break(0.4); }
    if mob.kind.id == "naturalist:jungle_scorpion" { return std::ops::ControlFlow::Break(0.7); }
    if matches!(mob.kind.id.as_str(), "naturalist:bear" | "naturalist:black_bear") {
        return std::ops::ControlFlow::Break(if mob.is_baby() { 0.225 } else { 0.9 });
    }
    if mob.kind.id == "naturalist:zebra" {
        return std::ops::ControlFlow::Break(if mob.is_baby() { 0.275 } else { 1.1 });
    }
    if mob.kind.id == "naturalist:rat" {
        return std::ops::ControlFlow::Break(if mob.is_baby() { 0.1 } else { 0.3 });
    }
    if mob.kind.id == "naturalist:snake" {
        return std::ops::ControlFlow::Break(if mob.is_baby() { 0.1 } else { 0.4 });
    }
    if matches!(mob.kind.id.as_str(), "naturalist:tiger" | "naturalist:lion") {
        return std::ops::ControlFlow::Break(if mob.is_baby() { 0.275 } else { 1.1 });
    }
    if mob.kind.id == "naturalist:tortoise" {
        return std::ops::ControlFlow::Break(if mob.is_baby() { 0.2 } else { 0.8 });
    }
    if mob.kind.id == "naturalist:rhino" {
        return std::ops::ControlFlow::Break(if mob.is_baby() { 0.55 } else { 1.1 });
    }
    if mob.kind.id == "naturalist:komodo_dragon" {
        return std::ops::ControlFlow::Break(if mob.is_baby() { 0.15 } else { 0.65 });
    }
    if mob.kind.id == "naturalist:capybara" {
        return std::ops::ControlFlow::Break(if mob.is_baby() { 0.125 } else { 0.5 });
    }
    if mob.kind.id == "naturalist:boar" {
        return std::ops::ControlFlow::Break(if mob.is_baby() { 0.175 } else { 0.7 });
    }
    if mob.kind.id == "naturalist:turkey" {
        return std::ops::ControlFlow::Break(if mob.is_baby() { 0.075 } else { 0.3 });
    }
    if mob.kind.id == "naturalist:deer" {
        let radius = if mob.is_baby() { 0.2 } else { 0.8 };
        return std::ops::ControlFlow::Break(radius);
    }
    std::ops::ControlFlow::Continue(())
}
