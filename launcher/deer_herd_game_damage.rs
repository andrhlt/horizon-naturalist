fn deer_herd_game_damage(game: &mut Game<'_>, id: u32) {
    super::naturalist_deer_herd::flush_hit(game, id);
}
