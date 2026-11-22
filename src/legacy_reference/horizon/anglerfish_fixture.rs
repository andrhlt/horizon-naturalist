fn anglerfish_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    use horizon_client_lib::mobs::Who;
    let Some(tag) = command.strip_prefix("naturalist_fixture_anglerfish ") else {
        return ControlFlow::Continue(());
    };
    let text = if tag == "deep" {
        game.naturalist_fixture_anglerfish_deep()
    } else if tag == "tables" {
        game.naturalist_fixture_aquatic_tables()
    } else if tag == "limits" {
        game.naturalist_fixture_aquatic_limits()
    } else if tag == "bass_termination" {
        game.naturalist_fixture_bass_termination()
    } else if tag == "placement" {
        let placement = |pos| super::natural_spawning::anglerfish_spawn::placement(game, pos);
        format!("{} {} {} {}", placement([8, 27, 8]), placement([20, 27, 8]), placement([8, 40, 8]), placement([8, 54, 8]))
    } else {
        let mob = game.mobs.iter().find(|mob| {
            game.tags_of(Target::Mob(mob.id)).iter().any(|candidate| candidate == tag)
        }).unwrap();
        match mob.ai.target {
            Some(Who::Mob(id)) => game.tags_of(Target::Mob(id)).into_iter().next().unwrap_or_default(),
            _ => "no target".to_owned(),
        }
    };
    game.feedback(Text::literal(format!("Anglerfish fixture {text}")));
    ControlFlow::Break(true)
}
