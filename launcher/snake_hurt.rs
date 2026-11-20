fn snake_hurt(mob: &mut Mob) {
    use self::goals::naturalist_snake_goals as snake;
    if mob.kind.id == snake::KIND
        && let Some(stack) = mob.extra.main_hand_stack().cloned()
    {
        snake::state_mut(&mut mob.ai.nav).pending_spit = Some(stack);
        mob.extra.set_main_hand_stack(None);
    }
}
