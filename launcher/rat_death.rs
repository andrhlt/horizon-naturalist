fn rat_death(game: &mut Game<'_>, mob: &Mob) {
    use horizon_client::mobs::goals::naturalist_rat_goals as rat;
    if mob.kind.id == rat::KIND && let Some(index) = game.mobs.iter().position(|current| current.id == mob.id) {
        super::naturalist_rat_work::spill(game, index, mob.position);
    }
}
