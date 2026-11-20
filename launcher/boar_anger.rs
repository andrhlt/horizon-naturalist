fn boar_anger(mob: &mut Mob, cx: &Cx<'_, '_>) {
    if mob.kind.id != "naturalist:boar" {
        return;
    }
    if mob.health > mob.attrs.max_health * 0.5 {
        super::neutral_anger::resume_anger(mob, cx.w.game_time);
        super::tame::update_persistent_anger(mob, cx.w);
        super::neutral_anger::settle_anger_end(mob, cx.w.game_time);
    } else {
        mob.ai.target = None;
        mob.ai.last_hurt_by = None;
        mob.ai.last_hurt_by_ref = None;
        mob.extra.angry_at = None;
        super::neutral_anger::stop(mob);
    }
}
