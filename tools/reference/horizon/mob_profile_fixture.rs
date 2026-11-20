fn mob_profile_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::finalize;
    use std::collections::BTreeMap;
    use std::ops::ControlFlow;

    if command == "naturalist_fixture_zebra_roll" && game.options.headless {
        let mut random = horizon_core::random::LegacyRandom::new(42);
        let samples: Vec<_> = (0..8).map(|_| horizon_client::mobs::naturalist_zebra::roll(&mut random)).collect();
        let row = serde_json::json!(samples);
        game.feedback(Text::literal(format!("Naturalist zebra rolls {row}")));
        return ControlFlow::Break(true);
    }

    let (kind, age_profile) = if let Some(kind) = command.strip_prefix("naturalist_fixture_mob_age_profile ") {
        (kind, true)
    } else if let Some(kind) = command.strip_prefix("naturalist_fixture_mob_profile ") {
        (kind, false)
    } else {
        return ControlFlow::Continue(());
    };
    if !game.options.headless {
        return ControlFlow::Continue(());
    }
    let Some(mob) = game.mobs.iter().find(|mob| mob.kind.id == kind && mob.alive()) else {
        return ControlFlow::Break(false);
    };
    if age_profile {
        let row = serde_json::json!({"dimensions": mob.dims(), "baby": mob.is_baby()});
        game.feedback(Text::literal(format!("Naturalist mob age profile {row}")));
        return ControlFlow::Break(true);
    }
    let values: BTreeMap<_, _> = [
        "attack_damage", "attack_knockback", "follow_range", "knockback_resistance",
        "max_health", "movement_speed", "scale", "step_height",
    ]
    .into_iter()
    .map(|name| {
        let value = finalize::command_attr(name)
            .filter(|attribute| finalize::default_value(mob, *attribute).is_some())
            .map(|attribute| finalize::base_value(mob, attribute));
        (name, value)
    })
    .collect();
    let row = serde_json::json!({
        "dimensions": mob.dims(),
        "category": &mob.kind.category,
        "attributes": values,
    });
    game.feedback(Text::literal(format!("Naturalist mob profile {row}")));
    ControlFlow::Break(true)
}
