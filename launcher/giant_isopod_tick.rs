fn giant_isopod_tick(kept: bool, mob: &mut Mob, events: &mut Vec<MobEvent>) -> bool {
    use super::nav::naturalist_giant_isopod as isopod;

    if kept && mob.alive() && super::nav::naturalist_crab::immobilized(mob) {
        mob.ai.jumping = false;
        mob.nav_stop();
    }
    if !kept || mob.kind.id != isopod::KIND || !mob.alive() {
        return kept;
    }
    let state = isopod::state_mut(&mut mob.ai.nav);
    state.hold = state.hold.saturating_sub(1);
    let was_hiding = state.was_hiding;
    let hiding = isopod::hiding(mob);
    if hiding {
        mob.ai.jumping = false;
        mob.nav_stop();
        if !was_hiding && !mob.silent {
            events.push(MobEvent::SoundId("naturalist:entity.giant_isopod.roll".to_owned(), mob.position.components(), 0.6, 1.0));
        }
    }
    isopod::state_mut(&mut mob.ai.nav).was_hiding = hiding;
    kept
}
