fn plush_bear_rest(problem: Option<String>, game: &Game<'_>) -> Option<String> {
    use horizon_client::mobs::Species;

    if problem.as_deref() != Some("not_safe") {
        return problem;
    }
    let holding_bear = [game.inventory[game.selected].as_ref(), game.offhand.as_ref()]
        .into_iter()
        .flatten()
        .any(|stack| stack.item.as_ref() == "naturalist:plush_bear");
    if !holding_bear {
        return problem;
    }
    let Some(player) = game.player.as_ref() else {
        return problem;
    };
    // Zombified Piglin overrides Monster.isPreventingPlayerRest in Java.
    // Keep Horizon's existing rest check for that species.
    let angry_piglin = game.mobs.iter().any(|mob| {
        mob.species() == Species::ZombifiedPiglin
            && mob.extra.anger > 0
            && mob.ai.target.is_some()
            && mob.alive()
            && (mob.position.x - player.position.x).abs() < 8.0
            && (mob.position.y - player.position.y).abs() < 5.0
            && (mob.position.z - player.position.z).abs() < 8.0
    });
    if angry_piglin { problem } else { None }
}
