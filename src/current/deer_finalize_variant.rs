fn deer_finalize_variant(
    game: &mut crate::game::Game<'_>,
    mob: &horizon_client::mobs::Mob,
    group: &mut Option<horizon_client::mobs::finalize::Group>,
    reason: horizon_client::mobs::spawn_rules::Reason,
) {
    if reason == horizon_client::mobs::spawn_rules::Reason::Natural {
        crate::game::naturalist_shared_spawn::before_grouped(game, mob, group);
        if mob.kind.id == "naturalist:deer" {
            *group = Some(horizon_client::mobs::finalize::Group::Ageable(
                horizon_client::mobs::finalize::Ageable::chance(0.05),
            ));
        }
    }
    crate::game::naturalist_deer_goals::choose_spawn_variant(game, mob, false);
}
