fn wing_register(m: &mut Mob) {
    match m.kind.id.as_str() {
        "naturalist:vulture" => super::goals::naturalist_vulture_goals::register(m),
        "naturalist:dragonfly" => super::nav::naturalist_dragonfly_movement::register(m),
        "naturalist:firefly" => super::goals::naturalist_firefly_behavior::register(m),
        _ => {}
    }
}
