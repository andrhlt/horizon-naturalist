fn deer_finalize_variant(game: &mut crate::game::Game<'_>, mob: &horizon_client::mobs::Mob) {
    crate::game::naturalist_deer_goals::choose_spawn_variant(game, mob, false);
}
