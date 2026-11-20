fn deer_eating_client_tick(mob: &mut Mob) {
    if mob.kind.id == "naturalist:deer" && mob.extra.anim_ticks > 0 {
        mob.extra.anim_ticks -= 1;
    }
}
