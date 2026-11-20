fn snake_register(mob: &mut Mob) {
    use super::goals::naturalist_snake_goals as snake;
    if mob.kind.id == snake::KIND {
        snake::register(mob);
    }
}
