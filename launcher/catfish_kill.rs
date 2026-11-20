fn catfish_kill(hurt: bool, game: &mut Game<'_>, target: u32, source: horizon_client::mobs::Damage) -> bool {
    use horizon_client::mobs::{Who, nav::naturalist_catfish_state as catfish};
    if hurt && let Some(Who::Mob(attacker)) = source.attacker
        && game.mobs.iter().any(|prey| prey.id == target && !prey.alive())
        && let Some(hunter) = game.mobs.iter_mut().find(|hunter| hunter.id == attacker && matches!(hunter.kind.id.as_str(), "naturalist:great_white_shark" | "naturalist:catfish" | "naturalist:anglerfish" | "naturalist:alligator" | "naturalist:tiger" | "naturalist:lion" | "naturalist:snake"))
    {
        catfish::set_cooldown(&mut hunter.ai.nav, catfish::COOLDOWN_TICKS);
    }
    hurt
}
