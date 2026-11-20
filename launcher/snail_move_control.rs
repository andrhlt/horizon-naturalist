fn snail_move_control(mob: &mut Mob) {
    if mob.kind.id == "naturalist:snail" {
        use super::naturalist_snail::snail_mut;
        let mut control = std::mem::take(&mut snail_mut(mob).control);
        control.tick(mob);
        snail_mut(mob).control = control;
    }
}
