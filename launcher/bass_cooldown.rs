fn bass_cooldown(mob: &mut Mob) {
    use crate::mobs::{control::MoveKind, nav::naturalist_bass_state as bass};
    if mob.kind.id != "naturalist:bass" {
        return;
    }
    let cooldown = bass::cooldown(&mob.ai.nav);
    if cooldown > 0 {
        bass::set_cooldown(&mut mob.ai.nav, cooldown - 1);
    }
    if let MoveKind::SmoothSwimming { max_turn_y, .. } = &mut mob.ai.mov.kind {
        *max_turn_y = if mob.extra.variant == 2 { 10.0 } else { 5.0 };
    }
}
