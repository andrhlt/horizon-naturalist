fn snake_held_speed(speed: f32, mob: &Mob) -> f32 {
    if mob.kind.id == super::goals::naturalist_snake_goals::KIND
        && mob.on_ground
        && mob.extra.main_hand.is_some()
    {
        speed * 0.5
    } else {
        speed
    }
}
