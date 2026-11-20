fn shark_speed_bonus(ordinary: f64, mob: &Mob) -> f64 {
    if mob.kind.id == super::nav::naturalist_shark_state::KIND && mob.ai.target.is_some() {
        ordinary * 1.4
    } else {
        ordinary
    }
}
