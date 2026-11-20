fn rhino_tick(mob: &mut Mob, _world: &MobWorld<'_>, _events: &mut Vec<MobEvent>) {
    if mob.kind.id == "naturalist:rhino" && mob.alive() {
        super::naturalist_rhino::tick_stun(mob);
        std::sync::Arc::make_mut(&mut mob.kind).max_head_y_rot = if mob.extra.sprinting { 1.0 } else { 50.0 };
    }
}
