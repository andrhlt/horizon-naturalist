fn giant_isopod_input(mut input: Vec3, mob: &Mob) -> Vec3 {
    use super::nav::naturalist_giant_isopod as isopod;

    if super::nav::naturalist_crab::immobilized(mob) {
        return Vec3::ZERO;
    }
    if mob.kind.id == isopod::KIND {
        let horizontal = if isopod::hiding(mob) {
            0.0
        } else if !mob.fluids.in_water {
            0.35
        } else {
            1.0
        };
        input.x *= horizontal;
        input.z *= horizontal;
    }
    input
}
