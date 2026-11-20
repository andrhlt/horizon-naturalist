fn crab_hit_mob(landed: bool, game: &mut Game<'_>, source: horizon_client::mobs::Damage) -> bool {
    if landed && let Some(horizon_client::mobs::Who::Mob(id)) = source.attacker {
        game.naturalist_crab_hit(id);
    }
    landed
}
