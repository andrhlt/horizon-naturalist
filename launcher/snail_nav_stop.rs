fn snail_nav_stop(mob: &mut Mob) {
    if mob.kind.id == "naturalist:snail" {
        use super::naturalist_snail::snail_mut;
        mob.ai.mov.operation = super::control::Operation::Wait;
        snail_mut(mob).crawl_target = None;
    }
}
