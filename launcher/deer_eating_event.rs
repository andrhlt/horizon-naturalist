fn deer_eating_event(mob: &mut Mob, event: u8) {
    if mob.kind.id == "minecraft:wolf" && matches!(event, 101 | 102) {
        super::goals::naturalist_mole_goals::state_mut(&mut mob.ai.nav).wolf_digging = event == 101;
    }

    if mob.kind.id == "naturalist:deer" && event == 10 {
        mob.extra.anim_ticks = 40;
    }
}
