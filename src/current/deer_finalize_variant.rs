fn deer_finalize_variant(
    game: &mut crate::game::Game<'_>,
    mob: &horizon_client::mobs::Mob,
    group: &mut Option<horizon_client::mobs::finalize::Group>,
    reason: horizon_client::mobs::spawn_rules::Reason,
) {
    if reason == horizon_client::mobs::spawn_rules::Reason::Natural {
        crate::game::naturalist_shared_spawn::before_grouped(game, mob, group);
    }
    crate::game::naturalist_deer_goals::choose_spawn_variant(game, mob, false);
}
