fn deer_finalize_variant(
    game: &mut crate::game::Game<'_>,
    mob: &horizon_client::mobs::Mob,
    group: &mut Option<horizon_client::mobs::finalize::Group>,
) {
    crate::game::naturalist_deer_goals::choose_spawn_variant(game, mob, false);
    crate::game::naturalist_shared_spawn::before_grouped(game, mob, group);
}
