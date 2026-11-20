fn anglerfish_tick(kept: bool, mob: &mut Mob) -> bool {
    use super::nav::naturalist_catfish_state as hunt;
    if kept && mob.alive() && mob.kind.id == "naturalist:anglerfish" {
        let cooldown = hunt::cooldown(&mob.ai.nav);
        if cooldown > 0 {
            hunt::set_cooldown(&mut mob.ai.nav, cooldown - 1);
        }
    }
    kept
}
