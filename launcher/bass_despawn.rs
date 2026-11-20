fn bass_despawn(mob: &mut Mob, world: &MobWorld<'_>) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if super::naturalist_config::removed(&mob.kind.id) {
        return ControlFlow::Break(false);
    }
    if !matches!(mob.kind.id.as_str(), "naturalist:bass" | "naturalist:catfish" | "naturalist:piranha" | "naturalist:blobfish" | "naturalist:jellyfish" | "naturalist:ray" | "naturalist:anglerfish" | "naturalist:starfish" | "naturalist:giant_isopod" | "naturalist:clam") {
        return ControlFlow::Continue(());
    }
    if let Some(player) = world.nearest_player {
        let distance = mob.distance_sqr(player);
        if distance > 128.0_f64.powi(2) {
            return ControlFlow::Continue(());
        }
        // Mob.checkDespawn makes the idle draw before removeWhenFarAway.
        if !mob.persistent && !requires_custom_persistence(mob) && mob.ai.no_action_time > 600 {
            let _ = mob.random.next_int_bound(800);
        }
        if distance < 32.0_f64.powi(2) {
            mob.ai.no_action_time = 0;
        }
    }
    ControlFlow::Break(true)
}
