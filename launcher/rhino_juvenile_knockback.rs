fn rhino_juvenile_knockback(power: f64, mob: &Mob) -> f64 {
    if matches!(mob.kind.id.as_str(), "naturalist:hippo" | "naturalist:alligator" | "naturalist:rhino" | "naturalist:bear" | "naturalist:black_bear" | "naturalist:tortoise" | "naturalist:elephant" | "naturalist:mammoth") && mob.is_baby() {
        power / (1.0 - mob.knockback_resistance()).max(0.01)
    } else if mob.kind.id == "naturalist:tortoise" && (mob.extra.sitting_pose || self::goals::naturalist_tortoise_goals::hidden(mob)) {
        power * 0.25
    } else {
        power
    }
}
