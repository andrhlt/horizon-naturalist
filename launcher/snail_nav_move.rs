fn snail_nav_move(started: bool, mob: &mut Mob, at: [f64; 3], speed: f64) -> bool {
    started
        || mob.kind.id == "naturalist:snail"
            && super::naturalist_snail_control::crawl_to(mob, at.map(|n| n.floor() as i32), speed)
}
