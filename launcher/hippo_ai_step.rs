fn hippo_ai_step(keep: bool, mob: &mut Mob) -> bool {
    use super::nav::naturalist_hippo_state as hippo;
    if !keep || mob.kind.id != hippo::KIND || !mob.alive() {
        return keep;
    }
    if mob.fluids.in_water && mob.horizontal_collision && !mob.ai.nav.is_done() {
        mob.velocity.y = mob.velocity.y.max(0.3);
    }
    let state = hippo::state_mut(&mut mob.ai.nav);
    if state.eating > 0 {
        state.eating -= 1;
    } else {
        mob.extra.set_main_hand_stack(None);
    }
    keep
}
