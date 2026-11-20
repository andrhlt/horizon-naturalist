fn giant_isopod_damage(amount: f32, mob: &Mob) -> f32 {
    if nav::naturalist_giant_isopod::hiding(mob) {
        amount * 0.5
    } else if nav::naturalist_crab::hides(mob) {
        amount * 0.8
    } else {
        amount
    }
}
