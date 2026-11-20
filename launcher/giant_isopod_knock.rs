fn giant_isopod_knock(power: f64, mob: &Mob) -> f64 {
    if nav::naturalist_giant_isopod::hiding(mob) || nav::naturalist_crab::hides(mob) { power * 0.25 } else { power }
}
