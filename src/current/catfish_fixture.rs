fn catfish_attack_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::{Damage, DamageKind, Species, Who};
    use std::ops::ControlFlow;

    if command != "naturalist_fixture_catfish_strike" {
        return ControlFlow::Continue(());
    }
    let Some(hunter) = game.mobs.iter().find(|m| m.kind.id == "naturalist:catfish" && m.alive()).map(|m| m.id) else {
        game.feedback(Text::literal("Catfish fixture missing hunter"));
        return ControlFlow::Break(true);
    };
    let Some(victim) = game.mobs.iter().find(|m| m.kind.id == "naturalist:bass" && m.alive()).map(|m| m.id) else {
        game.feedback(Text::literal("Catfish fixture missing Bass prey"));
        return ControlFlow::Break(true);
    };
    let source = Damage {
        kind: DamageKind::MobAttack,
        attacker: Some(Who::Mob(hunter)),
        from: None,
        attacker_species: Some(Species::Cod),
    };
    let landed = game.mob_attacks_mob(victim, 1.0, source);
    game.feedback(Text::literal(format!("Catfish strike {landed}")));
    ControlFlow::Break(true)
}
