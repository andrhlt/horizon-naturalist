fn snake_spawn(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    use super::goals::naturalist_snake_goals as snake;
    if mob.kind.id != snake::KIND {
        return std::ops::ControlFlow::Continue(());
    }
    snake::spawn_equipment(mob);
    let mut random = mob.random.clone();
    super::finalize::mob_spawn(mob, &mut random);
    mob.random = random;
    std::ops::ControlFlow::Break(())
}
