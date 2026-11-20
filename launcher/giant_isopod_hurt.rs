fn giant_isopod_hurt(mob: &mut Mob) {
    if nav::naturalist_giant_isopod::hiding(mob) {
        nav::naturalist_giant_isopod::state_mut(&mut mob.ai.nav).hold = 10;
    }
}
