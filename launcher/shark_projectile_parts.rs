fn shark_projectile_parts(mut targets: Vec<(Who, Aabb)>, game: &Game<'_>) -> Vec<(Who, Aabb)> {
    use horizon_client::mobs::nav::{naturalist_shark_state as shark, naturalist_whale_state as whale};
    for mob in game
        .mobs
        .iter()
        .filter(|mob| mob.alive())
    {
        let target = |part| (Who::Mob(mob.id), part);
        match mob.kind.id.as_str() {
            shark::KIND => targets.extend(game.shark_part_boxes(mob).map(target)),
            whale::KIND => targets.extend(game.whale_part_boxes(mob).map(target)),
            _ => (),
        }
    }
    targets
}
