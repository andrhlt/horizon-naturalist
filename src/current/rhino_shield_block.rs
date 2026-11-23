fn rhino_shield_block(
    game: &mut Game<'_>,
    attack: &horizon_client::health::Attack,
    outcome: BlockOutcome,
) {
    if outcome.blocked <= 0.0 || attack.attacker <= 0 { return; }
    crate::game::naturalist_rhino_goals::stun(game, attack.attacker as u32);
}
