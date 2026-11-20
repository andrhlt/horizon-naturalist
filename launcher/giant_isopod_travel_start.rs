fn giant_isopod_travel_start(mob: &mut Mob) {
    if super::nav::naturalist_giant_isopod::hiding(mob) || super::nav::naturalist_crab::immobilized(mob) {
        mob.velocity.x = 0.0;
        mob.velocity.z = 0.0;
    }
}
