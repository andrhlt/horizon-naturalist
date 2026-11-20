fn lion_created(game: &mut Game<'_>, from: usize) {
    use horizon_client::mobs::goals::naturalist_lion_goals as lion;

    for index in from..game.mobs.len() {
        if game.mobs[index].kind.id != lion::KIND || !lion::has_pending_spawn(&game.mobs[index]) {
            continue;
        }
        super::naturalist_deer_variants::assign_variant_at(game, index);
        lion::finish_spawn(&mut game.mobs[index], &mut game.world.random);
    }
}
