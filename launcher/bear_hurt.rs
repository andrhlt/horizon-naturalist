fn bear_hurt(mob: &mut Mob) {
    if !naturalist_bears::is_bear(&mob.kind.id) || !mob.alive() {
        return;
    }
    let pose = naturalist_bears::state_mut(&mut mob.ai.nav);
    pose.sleeping = false;
    pose.wake = 200;
    if let Some(stack) = mob.extra.main_hand_stack().cloned() {
        pose.pending_spit = Some(stack);
        mob.extra.set_main_hand_stack(None);
    }
}
