fn deer_sprinting(mob: &mut Mob) {
    let movement = &mob.ai.mov;
    if matches!(mob.kind.id.as_str(), "naturalist:elephant" | "naturalist:mammoth") {
        mob.extra.sprinting = !super::equine::is_vehicle(mob)
            && movement.has_wanted()
            && movement.speed_modifier > 1.0;
        return;
    }
    let threshold = match mob.kind.id.as_str() {
        "naturalist:deer" => 1.5,
        "naturalist:duck" | "naturalist:boar" => 1.2,
        "naturalist:capybara" => 1.3,
        "naturalist:rhino" => 1.5,
        "naturalist:bear" | "naturalist:black_bear" => 1.25,
        "naturalist:komodo_dragon" if mob.on_ground => 1.05,
        "naturalist:komodo_dragon" => f64::INFINITY,
        "naturalist:turkey" => 1.4,
        _ => return,
    };
    mob.extra.sprinting = movement.has_wanted() && movement.speed_modifier >= threshold;
}
